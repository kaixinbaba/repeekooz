use std::thread;

use bytes::{Buf, BufMut, BytesMut};
use tokio::io::{self, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::constants::{Error, States};
use crate::protocol::req::{ConnectRequest, ReqPacket, RequestHeader, DEATH_PTYPE};
use crate::protocol::resp::{ConnectResponse, ReplyHeader};
use crate::protocol::{Deserializer, Serializer};
use crate::{ZKError, ZKResult};

struct SenderTask {
    packet_rx: Receiver<ReqPacket>,
    writer: WriteHalf<TcpStream>,
}

impl SenderTask {
    pub(self) async fn run(&mut self) -> Result<(), io::Error> {
        loop {
            let packet = match self.packet_rx.recv().await {
                Some(packet) if packet.ptype != DEATH_PTYPE => packet,
                Some(_death) => {
                    info!("Received DEATH REQ quit!");
                    return Ok(());
                }
                None => continue,
            };
            let mut buf = BytesMut::new();
            if let Some(rh) = packet.rh {
                rh.write(&mut buf);
            }
            buf.extend(packet.req);
            self.writer.write_buf(&mut Client::wrap_len_buf(buf)).await;
            self.writer.flush().await;
        }
    }
}

struct ReceiverTask {
    buf_tx: Sender<(ReplyHeader, BytesMut)>,
    reader: ReadHalf<TcpStream>,
    is_connected: bool,
}

impl ReceiverTask {
    pub(self) async fn run(&mut self) -> Result<(), io::Error> {
        loop {
            // FIXME 1024 够吗
            let mut buf = BytesMut::with_capacity(1024);
            loop {
                let buf_size = match self.reader.read_buf(&mut buf).await {
                    Ok(buf_size) => buf_size,
                    _ => break,
                };
                if buf_size > 0 {
                    // skip first size
                    buf.get_i32();
                    break;
                }
            }
            let mut reply_header = ReplyHeader::default();
            if self.is_connected {
                reply_header.read(&mut buf);
                self.buf_tx.send((reply_header, buf)).await;
            } else {
                // connect response 没有 ReplyHeader
                self.buf_tx.send((reply_header, buf)).await;
                self.is_connected = true;
            }
        }
    }
}

struct EventTask {
    rx: Receiver<String>,
}

impl EventTask {
    pub(self) fn run(&self) {
        info!("in event loop");
    }
}

#[derive(Debug)]
struct HostProvider {
    server_list: Vec<String>,
    current_index: usize,
    server_len: usize,
}

impl HostProvider {
    fn validate_host(host: &str) -> ZKResult<String> {
        let ip_port = host.split(":").collect::<Vec<&str>>();
        if ip_port.len() != 2 {
            return Err(ZKError(
                Error::BadArguments,
                "Invalid host address format must be 'ip:port'",
            ));
        }
        match ip_port.get(1).unwrap().parse::<usize>() {
            Ok(port) if port <= 65535 => port,
            _ => {
                return Err(ZKError(
                    Error::BadArguments,
                    "Invalid port, must be number and less than 65535",
                ));
            }
        };

        for ip in ip_port.get(0).unwrap().split(".") {
            match ip.parse::<usize>() {
                Ok(i) if i > 255 => {
                    return Err(ZKError(
                        Error::BadArguments,
                        "ip address must between 0 and 255",
                    ));
                }
                Err(_) => return Err(ZKError(Error::BadArguments, "Invalid ip, must be number")),
                _ => (),
            }
        }
        Ok(host.to_string())
    }

    pub(self) fn new(connect_string: &str) -> ZKResult<(HostProvider, String)> {
        let split_chroot = connect_string.split("/").collect::<Vec<&str>>();
        if split_chroot.len() > 2 {
            return Err(ZKError(Error::BadArguments, "Invalid chroot format"));
        }
        let mut server_list = Vec::new();
        for add in split_chroot.get(0).unwrap().split(",") {
            match HostProvider::validate_host(add) {
                Ok(s) => server_list.push(s),
                Err(e) => return Err(e),
            }
        }

        let mut chroot = String::from("/");
        if split_chroot.len() == 2 {
            chroot = String::from("/".to_string() + *split_chroot.get(1).unwrap());
        }
        let server_len = server_list.len();
        Ok((
            HostProvider {
                server_list,
                current_index: 0,
                server_len,
            },
            chroot,
        ))
    }

    pub(self) fn pick_host(&mut self) -> &str {
        let address = self.server_list.get(self.current_index).unwrap();
        let mut new_index = self.current_index + 1;
        if new_index == self.server_len {
            new_index = 0;
        }
        self.current_index = new_index;

        address
    }
}

#[derive(Debug)]
pub(crate) struct Client {
    host_provider: HostProvider,
    session_timeout: i32,
    packet_tx: Sender<ReqPacket>,
    buf_rx: Receiver<(ReplyHeader, BytesMut)>,
    event_tx: Sender<String>,
    state: States,
    session_id: i64,
    password: Option<Vec<u8>>,
    chroot: String,
}

impl Client {
    pub(crate) fn get_path(&self, path: &str) -> String {
        let chroot = self.chroot.clone();
        let mut path = path.to_string();
        if !path.starts_with("/") {
            path = "/".to_string() + path.as_str();
        }
        if chroot != "/" {
            path = chroot + path.as_str()
        }
        path
    }

    pub(crate) async fn new(connect_string: &str, session_timeout: i32) -> ZKResult<Client> {
        let (mut host_provider, chroot) = HostProvider::new(connect_string)?;
        let socket = match TcpStream::connect(host_provider.pick_host()).await {
            Ok(socket) => socket,
            Err(_e) => return Err(ZKError(Error::BadArguments, "socket error")),
        };

        let (reader, writer) = io::split(socket);
        // start send thread
        let (packet_tx, packet_rx): (Sender<ReqPacket>, Receiver<ReqPacket>) = mpsc::channel(2017);
        let mut sender_task = SenderTask { packet_rx, writer };
        tokio::spawn(async move {
            sender_task.run().await;
            Ok::<_, io::Error>(())
        });

        // start receive thread
        let (buf_tx, buf_rx): (
            Sender<(ReplyHeader, BytesMut)>,
            Receiver<(ReplyHeader, BytesMut)>,
        ) = mpsc::channel(1002);
        let mut receiver_task = ReceiverTask {
            buf_tx,
            reader,
            is_connected: false,
        };
        tokio::spawn(async move {
            receiver_task.run().await;
            Ok::<_, io::Error>(())
        });

        // start event thread
        let (event_tx, event_rx) = mpsc::channel(2017);
        let event_task = EventTask { rx: event_rx };
        thread::spawn(move || event_task.run());

        let mut client = Client {
            host_provider,
            session_timeout,
            packet_tx,
            buf_rx,
            event_tx,
            state: States::NotConnected,
            session_id: 0,
            password: None,
            chroot,
        };
        client.start_connect(session_timeout).await?;
        client.state = States::Connected;
        Ok(client)
    }

    fn create_connect_request(&self, session_timeout: i32) -> ZKResult<BytesMut> {
        let mut buf = BytesMut::new();
        let connect_request = ConnectRequest::new(session_timeout);
        connect_request.write(&mut buf);
        Ok(buf)
    }

    async fn read_buf(&mut self) -> ZKResult<BytesMut> {
        let buf = match self.buf_rx.recv().await {
            Some((reply_header, buf)) => {
                if reply_header.err != 0 {
                    return Err(ZKError(
                        Error::from(reply_header.err as isize),
                        "occur error from server",
                    ));
                }
                buf
            }
            _ => return Err(ZKError(Error::ReadSocketError, "occur error from server")),
        };
        Ok(buf)
    }

    async fn write_buf(&mut self, rh: Option<RequestHeader>, req: BytesMut) -> ZKResult<()> {
        let packet = ReqPacket::new(rh, req);
        self.packet_tx.send(packet).await;
        Ok(())
    }

    async fn start_connect(&mut self, session_timeout: i32) -> ZKResult<()> {
        if self.state.is_connected() {
            return Ok(());
        }
        self.state = States::Connecting;

        // 创建 connect request
        let req = self.create_connect_request(session_timeout)?;

        self.write_buf(None, req).await?;

        let mut buf = self.read_buf().await?;

        let mut response = ConnectResponse::default();
        response.read(&mut buf)?;
        self.session_id = response.session_id;
        self.password = Some(response.password);
        Ok(())
    }

    pub async fn submit_request<D>(
        &mut self,
        rh: Option<RequestHeader>,
        req: BytesMut,
        mut resp: D,
    ) -> ZKResult<D>
    where
        D: Deserializer,
    {
        if !self.state.is_connected() {
            return Err(ZKError(Error::ConnectionLoss, "client may be closed"));
        }
        self.write_buf(rh, req).await?;
        let mut buf = self.read_buf().await?;
        resp.read(&mut buf);
        Ok(resp)
    }

    pub(crate) fn wrap_len_buf(buf: BytesMut) -> BytesMut {
        let len = buf.len();
        let mut wrap_buf = BytesMut::with_capacity(4 + len);
        wrap_buf.put_i32(len as i32);
        wrap_buf.extend(buf);
        wrap_buf
    }
}
