//! 测试的公共模块，存放测试用的公共函数


use tokio::io::{self, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio::prelude::{AsyncRead, AsyncWrite};

pub async fn try_connect_server(host: &str) -> (ReadHalf<TcpStream>, WriteHalf<TcpStream>) {
    let socket;
    match TcpStream::connect(String::from(host)).await {
        Ok(s) => socket = s,
        Err(e) => panic!("Can't connect to {}", host),
    }
    io::split(socket)
}




