#[macro_use]
extern crate log;

use bytes::{Buf, BufMut, BytesMut};
use tokio::prelude::*;

use buruma::protocol::{Deserializer, Serializer};
use buruma::protocol::req::{ConnectRequest, ReqPacket, CreateRequest, RequestHeader};
use buruma::protocol::resp::{ConnectResponse, ReplyHeader, CreateResponse};
use buruma::ZKResult;
use buruma::protocol::req::CreateMode::PERSISTENT;

mod common;

#[tokio::test]
async fn connect() {
    pretty_env_logger::init();
    let (mut r, mut w) = common::try_connect_server("127.0.0.1:2181").await;
    debug!("connect to server ok");
    let request = ConnectRequest::new();

    let mut p = ReqPacket::packet(None, request);
    w.write_buf(&mut p.bb.unwrap()).await;
    w.flush().await;

    let mut reader = BytesMut::with_capacity(4 * 1024);
    let buf_size = r.read_buf(&mut reader).await.unwrap();
    if buf_size != 0 {
        let i = reader.get_i32();
        if i != -1 {
            let mut response = ConnectResponse::default();
            response.read(&mut reader);
            info!("{:?}", response);
        }
    }
}

#[tokio::test]
async fn create() {
    pretty_env_logger::init();
    let (mut r, mut w) = common::try_connect_server("127.0.0.1:2181").await;
    debug!("connect to server ok");
    let path = "/xjj";
    let request = CreateRequest::new(path);

    let header = RequestHeader::new(0, 1);

    let mut p = ReqPacket::packet(Some(header), request);
    w.write_buf(&mut p.bb.unwrap()).await;
    w.flush().await;

    let mut reader = BytesMut::with_capacity(4 * 1024);
    let buf_size = r.read_buf(&mut reader).await.unwrap();
    if buf_size != 0 {
        let i = reader.get_i32();
        info!("{}", i);
        if i != -1 {
            let mut response = CreateResponse::default();
            response.read(&mut reader);
            info!("{:?}", response);
            assert_eq!(path, response.path);
        }
    }
}