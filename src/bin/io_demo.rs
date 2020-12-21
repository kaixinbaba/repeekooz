extern crate log;

use std::io::Write;
use std::thread::sleep;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::time::Duration;

use buruma::ZKResult;

#[tokio::main]
async fn main() -> ZKResult<()> {
    pretty_env_logger::init();

    let socket = TcpStream::connect("127.0.0.1:2182").await.unwrap();

    let (mut rd, mut wr) = io::split(socket);

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(1));
            println!("writing!!!");
            wr.write_all(b"hello\r\n").await?;
            wr.write_all(b"world\r\n").await?;
        }
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await.unwrap();

        if n == 0 {
            break;
        }
        let vec1 = Vec::from(&buf[..n]);
        let result = String::from_utf8(vec1).unwrap();

        println!("GOT {:?}", result);
    }

    Ok(())
}