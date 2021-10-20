extern crate chrono;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Release;

use bytes::{Buf, BufMut, BytesMut};
use chrono::prelude::*;
use futures_timer::Delay;
use tokio::io::{self, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::Duration;

use crate::{WatchedEvent, Watcher, ZKError, ZKResult};
use crate::constants::{OpCode, States, XidType};
use crate::error::ServerErrorCode;
use crate::metric::Metrics;
use crate::protocol::{Deserializer, Serializer};
use crate::protocol::req::{ConnectRequest, DEATH_PTYPE, ReqPacket, RequestHeader};
use crate::protocol::resp::{ConnectResponse, ReplyHeader, WatcherEvent};
use crate::watcher::WatcherManager;

struct SenderTask {
    packet_rx: Receiver<ReqPacket>,
    writer: WriteHalf<TcpStream>,
    metrics: Arc<Mutex<Metrics>>,
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
            if let Some(req) = packet.req {
                buf.extend(req);
            }
            self.writer.write_buf(&mut Client::wrap_len_buf(buf)).await;
            self.writer.flush().await;
        }
    }
}

struct ReceiverTask {
    buf_tx: Sender<(ReplyHeader, BytesMut)>,
    reader: ReadHalf<TcpStream>,
    event_tx: Sender<WatchedEvent>,
    // FIXME 用这个字段区分是否连接，是否可靠，之后的重连怎么处理
    is_connected: bool,
}

impl ReceiverTask {
    async fn read_origin_buf(&mut self) -> BytesMut {
        // FIXME 1024 够吗
        let mut buf = BytesMut::with_capacity(1024);
        loop {
            let buf_size = match self.reader.read_buf(&mut buf).await {
                Ok(buf_size) => buf_size,
                _ => break,
            };
            if buf_size > 0 {
                // skip first size
                let len = buf.get_i32();
                let bytes_mut = buf.split_to(len as usize);
                return bytes_mut;
            }
        }
        buf
    }

    async fn handle_reply(&self, mut reply_header: ReplyHeader, mut buf: BytesMut) {
        reply_header.read(&mut buf);
        // 区分不同的 xid
        match XidType::from(reply_header.xid) {
            XidType::Notification => {
                let mut server_event = WatcherEvent::default();
                server_event.read(&mut buf);
                self.event_tx.send(WatchedEvent::from(server_event)).await;
            }
            XidType::Ping => {
                trace!("Received Ping from server");
            }
            XidType::AuthPacket => {}
            XidType::SetWatches => {}
            XidType::Response => {
                trace!("Received Response from server");
                self.buf_tx.send((reply_header, buf)).await;
            }
        }
    }

    pub(self) async fn run(&mut self) -> Result<(), io::Error> {
        loop {
            let buf = self.read_origin_buf().await;
            let reply_header = ReplyHeader::default();

            if !self.is_connected {
                // connect response 没有 ReplyHeader
                self.buf_tx.send((reply_header, buf)).await;
                self.is_connected = true;
            } else {
                self.handle_reply(reply_header, buf).await;
            }
        }
    }
}

struct EventTask {
    event_rx: Receiver<WatchedEvent>,
    watcher_manager: Arc<WatcherManager>,
}

impl EventTask {
    // async fn process_event(&self, event: WatchedEvent, watchers: Vec<Box<dyn Watcher>>) {
    //     for w in watchers {
    //         w.process(&event);
    //     }
    // }

    pub(self) async fn run(&mut self) -> Result<(), io::Error> {
        loop {
            let event = match self.event_rx.recv().await {
                Some(event) => event,
                None => continue,
            };
            let _watchers = self.watcher_manager.find_need_triggered_watchers(&event);
            // self.process_event(event, watchers).await;
        }
    }
}

struct PingTask {
    packet_tx: Sender<ReqPacket>,
    metrics: Arc<Mutex<Metrics>>,
    read_timeout: i64,
}

impl PingTask {
    fn create_ping_request(&self) -> ReqPacket {
        ReqPacket::new(
            Some(RequestHeader::new_full(XidType::Ping as i32, OpCode::Ping)),
            None,
        )
    }

    pub(self) async fn run(&mut self) -> Result<(), io::Error> {
        let max_idle = 10000;
        loop {
            let idle_time =
                Local::now().timestamp_millis() - self.metrics.lock().unwrap().last_send_timestamp;
            let next_ping = if idle_time > 1000 {
                self.read_timeout / 2 - idle_time - 1000
            } else {
                self.read_timeout / 2 - idle_time
            };
            if next_ping <= 0 || idle_time > max_idle {
                // send_ping
                self.packet_tx.send(self.create_ping_request()).await;
                self.metrics.lock().unwrap().send_done();
            } else {
                Delay::new(Duration::from_millis(self.read_timeout as u64)).await;
            }
        }
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
            return Err(ZKError::ArgumentError("host".into(), "Host Address format must be 'ip:port'".into()));
        }
        match ip_port.get(1).unwrap().parse::<usize>() {
            Ok(port) if port <= 65535 => port,
            _ => {
                return Err(ZKError::ArgumentError("port".into(), "Port must be number and less than 65535".into()));
            }
        };

        for ip in ip_port.get(0).unwrap().split(".") {
            match ip.parse::<usize>() {
                Ok(i) if i > 255 => {
                    return Err(ZKError::ArgumentError("ip".into(), "ip address must between 0 and 255".into()));
                }
                Err(_) => return Err(ZKError::ArgumentError("ip".into(), "Invalid ip, must be number".into())),
                _ => (),
            }
        }
        Ok(host.to_string())
    }

    pub(self) fn new(connect_string: &str) -> ZKResult<(HostProvider, String)> {
        let split_chroot = connect_string.split("/").collect::<Vec<&str>>();
        if split_chroot.len() > 2 {
            return Err(ZKError::ArgumentError("host with chroot".into(), "chroot format must be like 'ip:port/chroot'".into()));
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
    pub session_timeout: u32,
    packet_tx: Sender<ReqPacket>,
    buf_rx: Receiver<(ReplyHeader, BytesMut)>,
    pub state: States,
    pub session_id: i64,
    password: Option<Vec<u8>>,
    chroot: String,
    watcher_manager: Arc<WatcherManager>,
    xid: AtomicI32,
}

impl Client {
    pub(crate) fn register_data_watcher(
        &self,
        path: String,
        watcher: Box<dyn Watcher>,
    ) -> ZKResult<()> {
        self.watcher_manager.register_data_watcher(path, watcher)?;
        Ok(())
    }

    pub(crate) fn register_exists_watcher(
        &self,
        path: String,
        watcher: Box<dyn Watcher>,
    ) -> ZKResult<()> {
        self.watcher_manager
            .register_exists_watcher(path, watcher)?;
        Ok(())
    }

    pub(crate) fn register_child_watcher(
        &self,
        path: String,
        watcher: Box<dyn Watcher>,
    ) -> ZKResult<()> {
        self.watcher_manager.register_child_watcher(path, watcher)?;
        Ok(())
    }

    pub(crate) fn register_persistent_watcher(
        &self,
        path: String,
        watcher: Box<dyn Watcher>,
        recursive: bool,
    ) -> ZKResult<()> {
        self.watcher_manager
            .register_persistent_watcher(path, watcher, recursive)?;
        Ok(())
    }

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

    pub(crate) async fn new(connect_string: &str, session_timeout: u32) -> ZKResult<Client> {
        let (mut host_provider, chroot) = HostProvider::new(connect_string)?;
        let socket = match TcpStream::connect(host_provider.pick_host()).await {
            Ok(socket) => socket,
            Err(_e) => return Err(ZKError::NetworkError),
        };

        let (reader, writer) = io::split(socket);
        let metrics = Arc::new(Mutex::new(Metrics::default()));
        // start send thread
        let (packet_tx, packet_rx): (Sender<ReqPacket>, Receiver<ReqPacket>) = mpsc::channel(2017);
        let mut sender_task = SenderTask {
            packet_rx,
            writer,
            metrics: metrics.clone(),
        };
        tokio::spawn(async move {
            sender_task.run().await;
            Ok::<_, io::Error>(())
        });

        let watcher_manager = Arc::new(WatcherManager::new(false));

        // start event thread
        let (event_tx, event_rx) = mpsc::channel(2017);
        let mut event_task = EventTask {
            event_rx,
            watcher_manager: watcher_manager.clone(),
        };
        tokio::spawn(async move {
            event_task.run().await;
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
            event_tx,
            is_connected: false,
        };
        tokio::spawn(async move {
            receiver_task.run().await;
            Ok::<_, io::Error>(())
        });

        let packet_tx_ping = Sender::clone(&packet_tx);

        let mut client = Client {
            host_provider,
            session_timeout,
            packet_tx,
            buf_rx,
            state: States::NotConnected,
            session_id: 0,
            password: None,
            chroot,
            watcher_manager: watcher_manager.clone(),
            xid: AtomicI32::new(0),
        };
        client.start_connect(session_timeout).await?;
        client.state = States::Connected;
        // start ping task
        let mut ping_task = PingTask {
            packet_tx: packet_tx_ping,
            metrics: metrics.clone(),
            read_timeout: (session_timeout / 3 * 2) as i64,
        };

        tokio::spawn(async move {
            ping_task.run().await;
            Ok::<_, io::Error>(())
        });
        Ok(client)
    }

    fn create_connect_request(&self, session_timeout: u32) -> ZKResult<BytesMut> {
        let mut buf = BytesMut::new();
        let connect_request = ConnectRequest::new(session_timeout);
        connect_request.write(&mut buf);
        Ok(buf)
    }

    async fn read_buf(&mut self, xid: Option<i32>) -> ZKResult<BytesMut> {
        let buf = match self.buf_rx.recv().await {
            Some((reply_header, buf)) => {
                if reply_header.err != 0 {
                    return Err(ZKError::ServerError(ServerErrorCode::from(reply_header.err), reply_header.err));
                }
                if let Some(xid) = xid {
                    if reply_header.xid != xid {
                        return Err(ZKError::ServerError(ServerErrorCode::ConnectionLoss, reply_header.err));
                    }
                }
                buf
            }
            _ => return Err(ZKError::UnknownError),

        };
        Ok(buf)
    }

    async fn write_buf(
        &mut self,
        mut rh: Option<RequestHeader>,
        req: BytesMut,
    ) -> ZKResult<Option<i32>> {
        let xid = match rh.as_mut() {
            Some(mut header) => {
                header.xid = self.xid.fetch_add(1, Release);
                Some(header.xid)
            }
            _ => None,
        };
        let packet = ReqPacket::new(rh, Some(req));
        self.packet_tx.send(packet).await;
        Ok(xid)
    }

    async fn start_connect(&mut self, session_timeout: u32) -> ZKResult<()> {
        if self.state.is_connected() {
            return Ok(());
        }
        self.state = States::Connecting;

        // 创建 connect request
        let req = self.create_connect_request(session_timeout)?;

        self.write_buf(None, req).await?;

        let mut buf = self.read_buf(None).await?;

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
            return Err(ZKError::NetworkError);
        }
        let xid = self.write_buf(rh, req).await?;
        let mut buf = self.read_buf(xid).await?;
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
