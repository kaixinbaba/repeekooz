use std::thread::sleep;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufWriter, BufReader};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::time::Duration;

use buruma::protocol::req::ConnectRequest;
use buruma::ZKResult;
use bytes::{Bytes, BytesMut, BufMut};
use std::fs::read;
use std::io::Cursor;
use buruma::protocol::Serializer;

fn write_to_server<W>(s: W, mut b: BytesMut) -> BytesMut
    where W: Serializer {
    s.write(&mut b);
    b
}

#[tokio::main]
async fn main() -> ZKResult<()> {
    let socket;
    match TcpStream::connect("127.0.0.1:2181").await {
        Ok(s) => socket = s,
        Err(e) => return Err("connect error".to_string())
    }
    // let mut w = BufWriter::new(socket);
    let (mut r, mut w) = io::split(socket);

    let mut reader = BytesMut::with_capacity(4 * 1024);


    let handle = tokio::spawn(async move {
        loop {
            println!("writing!!!");
            let mut request: ConnectRequest = ConnectRequest::new();
            println!("{:#?}", request);
            sleep(Duration::from_secs(2));

            let mut bytes_mut  = BytesMut::new();

            bytes_mut = write_to_server(request, bytes_mut);

            let len = bytes_mut.len();
            let mut bytes_mut2 = BytesMut::with_capacity(4 + len);
            bytes_mut2.put_i32(len as i32);
            bytes_mut2.extend(bytes_mut);

            w.write_buf(&mut bytes_mut2).await?;

            w.flush().await;


            loop {
                let buf_size = r.read_buf(&mut reader).await?;
                println!("{}", buf_size);
                if buf_size != 0 {
                    let result = String::from_utf8(Vec::from(&reader[..])).unwrap();
                    reader.clear();
                    println!("{}", result);
                    break;
                } else {
                    sleep(Duration::from_secs(2));
                }
            }
        }
        Ok::<_, io::Error>(())
    });
    handle.await;


    Ok(())
}