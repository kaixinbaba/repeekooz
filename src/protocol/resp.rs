use bytes::BytesMut;

use crate::protocol::Deserializer;
use crate::ZKResult;

#[derive(Debug, Default)]
pub struct ReplyHeader {
    pub xid: i32,
    pub zxid: i64,
    pub err: i32,
}

impl Deserializer for ReplyHeader {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        self.xid = self.read_i32(b);
        self.zxid = self.read_i64(b);
        self.err = self.read_i32(b);
        Ok(())
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
        self.path = self.read_string(b);
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct IgnoreResponse {}

impl Deserializer for IgnoreResponse {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        Ok(())
    }
}
