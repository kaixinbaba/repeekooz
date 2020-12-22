use std::hash::Hasher;

use bytes::{BufMut, BytesMut};

use crate::protocol::Serializer;

#[derive(Debug, Default)]
pub struct RequestHeader {
    xid: i32,
    rtype: i32,
}

impl RequestHeader {
    pub fn new(xid: i32, rtype: i32) -> RequestHeader {
        RequestHeader {
            xid,
            rtype,
        }
    }
}

impl Serializer for RequestHeader {
    fn write(&self, b: &mut BytesMut) {
        Self::write_i32(self.xid, b);
        Self::write_i32(self.rtype, b)
    }
}

#[derive(Debug, Default)]
pub struct ConnectRequest {
    protocol_version: i32,
    last_zxid_seen: i64,
    time_out: i32,
    session_id: i64,
    passwd: Option<Vec<u8>>,
    read_only: bool,
}

impl ConnectRequest {
    pub fn new() -> Self {
        ConnectRequest {
            protocol_version: 0,
            last_zxid_seen: 0,
            time_out: 10000,
            session_id: 0,
            passwd: None,
            read_only: false,
        }
    }
}

impl Serializer for ConnectRequest {
    fn write(&self, b: &mut BytesMut) {
        Self::write_i32(self.protocol_version, b);
        Self::write_i64(self.last_zxid_seen, b);
        Self::write_i32(self.time_out, b);
        Self::write_i64(self.session_id, b);
        Self::write_slice_option(self.passwd.clone(), b);
        Self::write_bool(self.read_only, b);
    }
}

#[derive(Debug, Default)]
pub struct ACL {
    perms: i32,
    scheme: String,
    id: String,
}

impl Serializer for ACL {
    fn write(&self, b: &mut BytesMut) {
        Self::write_i32(self.perms, b);
        Self::write_string(self.scheme.as_str(), b);
        Self::write_string(self.id.as_str(), b);
    }
}

#[derive(Debug, Default)]
pub struct CreateRequest {
    path: String,
    data: Option<Vec<u8>>,
    acl: Vec<ACL>,
    flags: i32,
}

impl Serializer for CreateRequest {
    fn write(&self, b: &mut BytesMut) {
        Self::write_string(self.path.as_str(), b);
        Self::write_slice_option(self.data.clone(), b);
        Self::write_vec(&self.acl, b);
    }
}

impl CreateRequest {
    pub fn new(path: &str) -> Self {
        let mut acl = Vec::with_capacity(1);
        acl.push(ACL {
            perms: 31,
            scheme: "world".to_string(),
            id: "anyone".to_string(),
        });
        CreateRequest {
            path: String::from(path),
            data: None,
            acl,
            flags: CreateMode::PERSISTENT as i32,
        }
    }
}

pub enum CreateMode {
    PERSISTENT = 0,
}


#[derive(Debug, Default)]
pub struct ReqPacket<S: Serializer> {
    pub rh: Option<RequestHeader>,
    pub req: S,
    pub bb: Option<BytesMut>,
}

impl<S: Serializer> ReqPacket<S> {
    pub fn new(rh: Option<RequestHeader>, req: S) -> Self {
        ReqPacket {
            rh,
            req,
            bb: None
        }
    }

    pub fn packet(rh: Option<RequestHeader>, req: S) -> Self {
        let mut p = Self::new(rh, req);
        let mut b = BytesMut::new();
        p.write(&mut b);

        let len = b.len();
        let mut bb = BytesMut::with_capacity(4 + len);
        bb.put_i32(len as i32);
        bb.extend(b);
        p.bb = Some(bb);
        p
    }
}

impl<S: Serializer> Serializer for ReqPacket<S> {
    fn write(&self, b: &mut BytesMut) {
        if let Some(rh) = &self.rh {
            rh.write(b);
        }
        self.req.write(b);
    }
}
