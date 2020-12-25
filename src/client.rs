use std::thread;

use bytes::{Buf, BufMut, BytesMut};
use tokio::io::{self, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::time::Duration;

use crate::constants::{Error, States};
use crate::protocol::{Deserializer, Serializer};
use crate::protocol::req::{ConnectRequest, ReqPacket};
use crate::protocol::resp::ConnectResponse;
use crate::ZKResult;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender, Receiver};

struct SenderTask {
    rx: Receiver<ReqPacket>,
    writer: WriteHalf<TcpStream>,
}

impl SenderTask {
    pub(self) async fn run(&mut self) -> Result<(), io::Error> {
        loop {
            info!("send loop");
            thread::sleep(Duration::from_secs(1));
            let mut packet = match self.rx.recv().await {
                Some(packet) => packet,
                None => {
                    error!("send run got None");
                    continue;
                },
            };
            info!("{:?}", packet);
            self.writer.write_buf(&mut packet.req).await;
            self.writer.flush().await;
            info!("flush done loop");
        }
        Ok(())
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
    // event_tx: Sender<String>,
    state: States,
    session_id: i64,
}

impl Client {
    pub async fn new(connect_string: &str, session_timeout: i32) -> ZKResult<Client> {
        let mut server_list = Vec::new();
        server_list.push(connect_string.to_string());
        let socket = match TcpStream::connect(server_list.get(0).unwrap().as_str()).await {
            Ok(socket) => socket,
            Err(e) => return Err(Error::BadArguments),
        };

        let (mut reader, mut writer) = io::split(socket);
        // start send thread
        let (packet_tx, packet_rx): (Sender<ReqPacket>, Receiver<ReqPacket>) = mpsc::channel(1000);

        let mut sender_task = SenderTask {
            rx: packet_rx,
            writer,
        };
        tokio::spawn(async move {
            sender_task.run().await;
            Ok::<_, io::Error>(())
        });

        // start event thread
        // let (event_tx, event_rx) = mpsc::channel();
        // let event_task = EventTask {
        //     rx: event_rx,
        // };
        // thread::spawn(move || event_task.run());

        let mut client = Client {
            server_list,
            session_timeout,
            reader,
            packet_tx,
            // event_tx,
            state: States::NotConnected,
            session_id: 0,
        };
        client.start_connect(session_timeout).await?;
        client.state = States::Connected;
        Ok(client)
    }

    fn create_connect_request(&self, session_timeout: i32) -> ZKResult<ConnectRequest> {
        Ok(ConnectRequest::new(session_timeout))
    }

    async fn start_connect(&mut self, session_timeout: i32) -> ZKResult<()> {
        if self.state.is_connected() {
            return Ok(());
        }
        self.state = States::Connecting;

        // 创建 connect request
        let mut buf = BytesMut::new();
        let connect_request = self.create_connect_request(session_timeout)?;
        connect_request.write(&mut buf);
        let wrap_buf = Client::wrap_len_buf(buf);
        let packet = ReqPacket::new(None, wrap_buf);
        self.packet_tx.send(packet).await;
        let mut buf = BytesMut::with_capacity(2 * 1024);
        loop {
            let buf_size = match self.reader.read(&mut buf).await {
                Ok(buf_size) if buf_size >= 0 => buf_size,
                _ => return Err(Error::ReadSocketError),
            };
            info!("buf_size : {}", buf_size);

            if buf_size > 0 {
                break;
            }
            thread::sleep(Duration::from_secs(1));
        }
        let mut response = ConnectResponse::default();
        response.read(&mut buf)?;
        info!("{:?}", response);
        Ok(())
    }

    fn wrap_len_buf(buf: BytesMut) -> BytesMut {
        let len = buf.len();
        info!("{}", len);
        let mut wrap_buf = BytesMut::with_capacity(4 + len);
        wrap_buf.put_i32(len as i32);
        wrap_buf.extend(buf);
        info!("{}", wrap_buf.len());
        wrap_buf
    }
}

