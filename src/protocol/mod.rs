use bytes::{Buf, BufMut, BytesMut};

use crate::protocol::req::RequestHeader;
use crate::protocol::resp::ReplyHeader;
use crate::ZKResult;

pub mod req;
pub mod resp;

pub trait Serializer: Send {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()>;

    fn write_i32(&self, i: i32, b: &mut BytesMut) {
        b.put_i32(i);
    }

    fn write_i64(&self, i: i64, b: &mut BytesMut) {
        b.put_i64(i);
    }

    fn write_slice_option(&self, v: Option<Vec<u8>>, b: &mut BytesMut) {
        match v {
            Some(vv) => self.write_slice(vv, b),
            None => self.write_i32(-1, b),
        }
    }

    fn write_slice(&self, v: Vec<u8>, b: &mut BytesMut) {
        b.put_i32(v.len() as i32);
        b.put_slice(v.as_slice());
    }

    fn write_bool(&self, flag: bool, b: &mut BytesMut) {
        b.put_u8(flag as u8);
    }
}

pub trait Deserializer {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()>;
}

