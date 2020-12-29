use std::thread;

use bytes::{Buf, BufMut, BytesMut};
use tokio::io::{self, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::Duration;

use crate::constants::{Error, States};
use crate::protocol::{Deserializer, Serializer};
use crate::protocol::req::{ConnectRequest, DEATH_PTYPE, ReqPacket, RequestHeader};
use crate::protocol::resp::{ConnectResponse, ReplyHeader};
use crate::ZKResult;

struct SenderTask {
    rx: Receiver<ReqPacket>,
    writer: WriteHalf<TcpStream>,
}

impl SenderTask {
    pub(self) async fn run(&mut self) -> Result<(), io::Error> {
        loop {
            let mut packet = match self.rx.recv().await {
                Some(packet) if packet.ptype != DEATH_PTYPE => packet,
                Some(death) => {
                    info!("Received DEATH REQ quit!");
                    println!("Received DEATH REQ quit!");
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

struct EventTask {
    rx: Receiver<String>,
}

impl EventTask {
    pub(self) fn run(&self) {
        info!("in event loop");
    }
}

#[derive(Debug)]
pub(crate) struct Client {
    server_list: Vec<String>,
    session_timeout: i32,
    reader: ReadHalf<TcpStream>,
    packet_tx: Sender<ReqPacket>,
    event_tx: Sender<String>,
    state: States,
    session_id: i64,
    password: Option<Vec<u8>>,
}

impl Client {
    pub(crate) async fn new(connect_string: &str, session_timeout: i32) -> ZKResult<Client> {
        let mut server_list = Vec::new();
        server_list.push(connect_string.to_string());
        let socket = match TcpStream::connect(server_list.get(0).unwrap().as_str()).await {
            Ok(socket) => socket,
            Err(e) => return Err(Error::BadArguments),
        };

        let (mut reader, mut writer) = io::split(socket);
        // start send thread
        let (packet_tx, packet_rx): (Sender<ReqPacket>, Receiver<ReqPacket>) = mpsc::channel(17120);

        let mut sender_task = SenderTask {
            rx: packet_rx,
            writer,
        };
        tokio::spawn(async move {
            sender_task.run().await;
            Ok::<_, io::Error>(())
        });

        // start event thread
        let (event_tx, event_rx) = mpsc::channel(17120);
        let event_task = EventTask {
            rx: event_rx,
        };
        thread::spawn(move || event_task.run());

        let mut client = Client {
            server_list,
            session_timeout,
            reader,
            packet_tx,
            event_tx,
            state: States::NotConnected,
            session_id: 0,
            password: None,
        };
        client.start_connect(session_timeout).await?;
        client.state = States::Connected;
        Ok(client)
    }

    fn create_connect_request(&self, session_timeout: i32) -> ZKResult<BytesMut> {
        let mut buf = BytesMut::new();
        let connect_request = ConnectRequest::new(session_timeout);
        connect_request.write(&mut buf);
        Ok(Client::wrap_len_buf(buf))
    }

    async fn read_buf(&mut self) -> ZKResult<BytesMut> {
        // 1024 够吗
        let mut buf = BytesMut::with_capacity(1024);
        loop {
            let buf_size = match self.reader.read_buf(&mut buf).await {
                Ok(buf_size) => buf_size,
                _ => return Err(Error::ReadSocketError),
            };
            if buf_size > 0 {
                // skip first size
                buf.get_i32();
                break;
            }
        }
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

    pub async fn submit_request<D>(&mut self, rh: Option<RequestHeader>, req: BytesMut, mut resp: D) -> ZKResult<(ReplyHeader, D)>
        where D: Deserializer {
        if !self.state.is_connected() {
            return Err(Error::ConnectionLoss);
        }
        self.write_buf(rh, req).await?;
        let mut buf = self.read_buf().await?;
        let mut reply_header = ReplyHeader::default();
        reply_header.read(&mut buf);
        resp.read(&mut buf);
        Ok((reply_header, resp))
    }

    pub fn wrap_len_buf(buf: BytesMut) -> BytesMut {
        let len = buf.len();
        let mut wrap_buf = BytesMut::with_capacity(4 + len);
        wrap_buf.put_i32(len as i32);
        wrap_buf.extend(buf);
        wrap_buf
    }
}

