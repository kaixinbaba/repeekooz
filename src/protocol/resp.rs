use bytes::BytesMut;

use crate::protocol::Deserializer;
use crate::ZKResult;

#[derive(Debug, Default)]
pub struct ReplyHeader {
    xid: i32,
    zxid: i64,
    err: i32,
}

impl Deserializer for ReplyHeader {
    fn read(&mut self, b: &mut BytesMut) {
        self.xid = Self::read_i32(b);
        self.zxid = Self::read_i64(b);
        self.err = Self::read_i32(b);
    }
}

#[derive(Debug, Default)]
pub struct ConnectResponse {
    protocol_version: i32,
    time_out: i32,
    session_id: i64,
    password: Vec<u8>,
    read_only: bool,
}

impl Deserializer for ConnectResponse {
    fn read(&mut self, b: &mut BytesMut) {
        self.protocol_version = Self::read_i32(b);
        self.time_out = Self::read_i32(b);
        self.session_id = Self::read_i64(b);
        self.password = Self::read_slice_unchecked(b);
        self.read_only = Self::read_bool(b);
    }
}

#[derive(Debug, Default)]
pub struct CreateResponse {
    pub path: String,
}

impl Deserializer for CreateResponse {

    fn read(&mut self, b: &mut BytesMut) {
        self.path = Self::read_string(b);
    }
}