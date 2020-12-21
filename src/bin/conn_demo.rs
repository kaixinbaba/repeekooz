use std::thread::sleep;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufWriter, BufReader};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::time::Duration;

use buruma::ZKResult;
use bytes::{Bytes, BytesMut, BufMut};
use std::fs::read;
use std::io::Cursor;

#[derive(Debug)]
struct ConnectRequest {
    protocol_version: i32,
    last_zxid_seen: Option<i64>,
    time_out: i32,
    session_id: i64,
    passwd: Option<Vec<u8>>,
}

impl Default for ConnectRequest {
    fn default() -> Self {
        ConnectRequest {
            protocol_version: 0,
            last_zxid_seen: Some(0),
            time_out: 10000,
            session_id: 0,
            passwd: None,
        }
    }
}

#[tokio::main]
async fn main() -> ZKResult<()> {
    let socket;
    match TcpStream::connect("127.0.0.1:2182").await {
        Ok(s) => socket = s,
        Err(e) => return Err("connect error".to_string())
    }
    // let mut w = BufWriter::new(socket);
    let (mut r, mut w) = io::split(socket);
    let mut reader = BytesMut::with_capacity(4 * 1024);

    let handle = tokio::spawn(async move {
        loop {
            println!("writing!!!");
            let request = ConnectRequest::default();
            println!("{:#?}", request);
            sleep(Duration::from_secs(2));

            let mut bytes_mut  = BytesMut::new();

            bytes_mut.put_i32(request.protocol_version);
            bytes_mut.put_i64(request.last_zxid_seen.unwrap());
            bytes_mut.put_i32(request.time_out);
            bytes_mut.put_i64(request.session_id);
            // bytes_mut.put_slice(vec![].as_slice());
            bytes_mut.put_i32(-1);
            bytes_mut.put_u8(0);

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