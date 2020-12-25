use std::fs::read;
use std::io::Cursor;
use std::thread::sleep;

use bytes::{BufMut, Bytes, BytesMut, Buf};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter, WriteHalf, ReadHalf};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::time::Duration;

use buruma::protocol::req::{ConnectRequest, CreateRequest};
use buruma::protocol::Serializer;
use buruma::ZKResult;
use buruma::constants::Error;

fn write_to_server<W>(s: W, mut b: BytesMut) -> BytesMut
    where W: Serializer {
    s.write(&mut b);
    b
}

async fn send_conn(w: &mut WriteHalf<TcpStream>, r: &mut ReadHalf<TcpStream>) {
    let mut bytes_mut  = BytesMut::new();
    let request = ConnectRequest::new(10000);
    bytes_mut = write_to_server(request, bytes_mut);

    let len = bytes_mut.len();
    let mut bytes_mut2 = BytesMut::with_capacity(4 + len);
    bytes_mut2.put_i32(len as i32);
    bytes_mut2.extend(bytes_mut);

    w.write_buf(&mut bytes_mut2).await;

    w.flush().await;

    let mut reader = BytesMut::with_capacity(4 * 1024);

    let buf_size = r.read_buf(&mut reader).await.unwrap();
    println!("{}", buf_size);
    if buf_size != 0 {
        let i = reader.get_i32();
        if i != -1 {
            println!("read len {}", i);
            let protocol_version = reader.get_i32();
            let time_out = reader.get_i32();
            let session_id = reader.get_i64();
            let passwd_len = reader.get_i32();
            if passwd_len != -1 {
                let option = reader.get(..(passwd_len as usize));
                reader.advance(passwd_len as usize);
            }
            let read_only = reader.get_u8();
            println!("{}, {}, {}, {}, {}", protocol_version, time_out, session_id, passwd_len, read_only);
        }
        reader.clear();
    }
}

#[tokio::main]
async fn main() -> ZKResult<()> {
    let socket;
    match TcpStream::connect("127.0.0.1:2181").await {
        Ok(s) => socket = s,
        Err(e) => return Err(Error::BadArguments),
    }
    // let mut w = BufWriter::new(socket);
    let (mut r, mut w) = io::split(socket);

    let mut reader = BytesMut::with_capacity(4 * 1024);


    let handle = tokio::spawn(async move {
        loop {
            let request = CreateRequest::new("/xjj");
            sleep(Duration::from_secs(2));

            send_conn(&mut w, &mut r).await;
            println!("====================== connect done");
            let mut bytes_mut = BytesMut::new();

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
                    let i = reader.get_i32();
                    if i != -1 {
                        println!("read len {}", i);
                        let xid = reader.get_i32();
                        let zxid = reader.get_i64();
                        let err = reader.get_i32();
                        println!("{}, {}, {}", xid, zxid, err);
                        let s_len = reader.get_i32();
                        println!("s_len {}", s_len);
                        let x = String::from_utf8(Vec::from(reader.get(..(s_len as usize)).unwrap()));
                        println!("s: {:?}", x);
                    }
                    reader.clear();
                    break;
                } else {
                    sleep(Duration::from_secs(2));
                }
            }
            break;
        }
        Ok::<_, io::Error>(())
    });
    handle.await;


    Ok(())
}