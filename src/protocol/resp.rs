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
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        unimplemented!()
    }
}

#[derive(Debug, Default)]
pub struct ConnectResponse {
    protocol_version: i32,
    time_out: i32,
    pub session_id: i64,
    pub password: Vec<u8>,
    read_only: bool,
}

impl Deserializer for ConnectResponse {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        self.protocol_version = self.read_i32(b);
        self.time_out = self.read_i32(b);
        self.session_id = self.read_i64(b);
        self.password = self.read_slice_unchecked(b);
        self.read_only = self.read_bool(b);
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct CreateResponse {
    pub path: String,
}

impl Deserializer for CreateResponse {

    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        unimplemented!()
    }
}