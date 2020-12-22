use bytes::{Buf, BufMut, BytesMut};

use crate::protocol::req::RequestHeader;
use crate::protocol::resp::ReplyHeader;

pub mod req;
pub mod resp;

pub trait Serializer {
    fn write(&self, b: &mut BytesMut);

    fn write_bool(i: bool, b: &mut BytesMut) {
        b.put_u8(i as u8);
    }

    fn write_i32(i: i32, b: &mut BytesMut) {
        b.put_i32(i);
    }

    fn write_i64(i: i64, b: &mut BytesMut) {
        b.put_i64(i);
    }

    fn write_u32(i: u32, b: &mut BytesMut) {
        b.put_u32(i);
    }

    fn write_len(i: usize, b: &mut BytesMut) {
        b.put_u32(i as u32);
    }


    fn write_string(s: &str, b: &mut BytesMut) {
        b.put_i32(s.len() as i32);
        b.put_slice(s.as_bytes());
    }

    fn write_vec<S>(v: &Vec<S>, b: &mut BytesMut) where S: Serializer {
        if v.is_empty() {
            Self::write_i32(-1, b);
            return;
        }
        Self::write_len(v.len(), b);
        for s in v.iter() {
            s.write(b);
        }
    }

    fn write_slice_option(v: Option<Vec<u8>>, b: &mut BytesMut) {
        match v {
            Some(vv) => Self::write_slice(vv, b),
            None => Self::write_i32(-1, b),
        }
    }

    fn write_slice(v: Vec<u8>, b: &mut BytesMut) {
        b.put_i32(v.len() as i32);
        b.put_slice(v.as_slice());
    }
}

pub trait Deserializer {
    fn read(&mut self, b: &mut BytesMut);

    fn read_bool(b: &mut BytesMut) -> bool {
        b.get_u8() != 0
    }

    fn read_i32(b: &mut BytesMut) -> i32 {
        b.get_i32()
    }

    fn read_i64(b: &mut BytesMut) -> i64 {
        b.get_i64()
    }

    fn read_string(b: &mut BytesMut) -> String {
        let s_len = b.get_i32();
        String::from_utf8(Vec::from(b.get(..(s_len as usize)).unwrap())).unwrap()
    }

    fn read_slice_unchecked(b: &mut BytesMut) -> Vec<u8> {
        let len = b.get_i32();
        if len == -1 {
            return Vec::from([0; 16]);
        }
        let v = Vec::from(b.get(..(len as usize)).unwrap());
        b.advance(len as usize);
        v
    }
}

