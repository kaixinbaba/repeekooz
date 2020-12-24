use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;

use tokio::io::{self, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::time::Duration;

use crate::constants::{Error, States};
use crate::protocol::req::{ReqPacket, ConnectRequest};
use crate::ZKResult;

struct SenderTask {
    rx: Receiver<String>,
}

impl SenderTask {
    pub(self) fn run(&self) {
        loop {
            match self.rx.recv() {
                Ok(s) => println!("receive {}", s),
                Err(e) => println!("error {:?}", e),
            }
            info!("in loop");
            thread::sleep(Duration::from_secs(1));
        }
    }
}

struct EventTask {
    rx: Receiver<String>,
}

impl EventTask {
    pub(self) fn run(&self) {
        loop {
            match self.rx.recv() {
                Ok(s) => println!("receive {}", s),
                Err(e) => println!("error {:?}", e),
            }
            info!("in event loop");
            thread::sleep(Duration::from_secs(1));
        }
    }
}

#[derive(Debug)]
pub(crate) struct Client {
    server_list: Vec<String>,
    session_timeout: i32,
    reader: ReadHalf<TcpStream>,
    writer: WriteHalf<TcpStream>,
    packet_tx: Sender<String>,
    event_tx: Sender<String>,
    state: States,
}

impl Client {
    pub async fn new(connect_string: &str, session_timeout: i32) -> ZKResult<Client> {
        let mut server_list = Vec::new();
        server_list.push(connect_string.to_string());
        let socket = match TcpStream::connect(server_list.get(0).unwrap().as_str()).await {
            Ok(socket) => socket,
            Err(e) => return Err(Error::BadArguments),
        };

        // start send thread
        let (packet_tx, packet_rx) = mpsc::channel();
        let sender_task = SenderTask {
            rx: packet_rx,
        };
        thread::spawn(move || sender_task.run());

        // start event thread
        let (event_tx, event_rx) = mpsc::channel();
        let event_task = EventTask {
            rx: event_rx,
        };
        thread::spawn(move || event_task.run());

        let (reader, writer) = io::split(socket);
        Ok(Client {
            server_list,
            session_timeout,
            reader,
            writer,
            packet_tx,
            event_tx,
            state: States::NotConnected,
        })
    }

    fn create_connect_request(&self) -> ZKResult<ConnectRequest> {
        unimplemented!()
    }

    fn start_connect(&mut self) -> ZKResult<()> {
        self.state = States::Connecting;
        let connect_request = self.create_connect_request()?;

        Ok(())
    }

    pub fn run(mut self) {
        while self.state.is_alive() {
            // connect
            if !self.state.is_connected() {
                match self.start_connect() {
                    Ok(_) => self.state = States::Connected,
                    Err(e) => panic!("Connect failed to server!"),
                }
            }
        }
        error!("IO thread exit loop");
    }
}

