#![allow(unused)]
use bytes::{Buf, BufMut, BytesMut};

use crate::ZKResult;

pub mod req;
pub mod resp;

pub trait Serializer: Send {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()>;

    fn write_i32(&self, i: i32, b: &mut BytesMut) {
        b.put_i32(i);
    }

    fn write_u32(&self, i: u32, b: &mut BytesMut) {
        b.put_u32(i);
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

    fn write_string(&self, s: &str, b: &mut BytesMut) {
        b.put_i32(s.len() as i32);
        b.put_slice(s.as_bytes());
    }

    fn write_len(&self, i: usize, b: &mut BytesMut) {
        b.put_u32(i as u32);
    }

    fn write_vec<S>(&self, v: &[S], b: &mut BytesMut)
    where
        S: Serializer,
    {
        if v.is_empty() {
            self.write_i32(-1, b);
            return;
        }
        self.write_len(v.len(), b);
        for s in v.iter() {
            s.write(b);
        }
    }
}

pub trait Deserializer {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()>;

    fn read_bool(&mut self, b: &mut BytesMut) -> bool {
        b.get_u8() != 0
    }

    fn read_i32(&mut self, b: &mut BytesMut) -> i32 {
        b.get_i32()
    }

    fn read_u32(&mut self, b: &mut BytesMut) -> u32 {
        b.get_u32()
    }

    fn read_i64(&mut self, b: &mut BytesMut) -> i64 {
        b.get_i64()
    }

    fn read_u64(&mut self, b: &mut BytesMut) -> u64 {
        b.get_u64()
    }

    fn read_string(&mut self, b: &mut BytesMut) -> String {
        String::from_utf8(self.read_slice_unchecked(b)).unwrap()
    }

    fn read_slice_unchecked(&mut self, b: &mut BytesMut) -> Vec<u8> {
        let len = b.get_i32();
        if len == -1 {
            return Vec::from([0; 0]);
        }

        let arr = match b.get(..(len as usize)) {
            Some(arr) => arr,
            None => return Vec::from([0; 0]),
        };
        let v = Vec::from(arr);
        b.advance(len as usize);
        v
    }
}
