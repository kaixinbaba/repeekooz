use std::hash::Hasher;

use bytes::BytesMut;

use crate::constants::{CreateMode, Perms, ANYONE, DIGEST, IP, SUPER, WORLD};
use crate::protocol::Serializer;
use crate::ZKResult;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default)]
pub(crate) struct RequestHeader {
    xid: i32,
    rtype: i32,
}

impl RequestHeader {
    pub(crate) fn new(xid: i32, rtype: i32) -> RequestHeader {
        RequestHeader { xid, rtype }
    }
}

impl Serializer for RequestHeader {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_i32(self.xid, b);
        self.write_i32(self.rtype, b);
        Ok(())
    }
}

#[derive(Debug, Default)]
pub(crate) struct ConnectRequest {
    protocol_version: i32,
    last_zxid_seen: i64,
    time_out: u32,
    session_id: i64,
    passwd: Option<Vec<u8>>,
    read_only: bool,
}

impl ConnectRequest {
    pub(crate) fn new(session_timeout: u32) -> Self {
        ConnectRequest {
            protocol_version: 0,
            last_zxid_seen: 0,
            time_out: session_timeout,
            session_id: 0,
            passwd: None,
            read_only: false,
        }
    }
}

impl Serializer for ConnectRequest {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_i32(self.protocol_version, b);
        self.write_i64(self.last_zxid_seen, b);
        self.write_u32(self.time_out, b);
        self.write_i64(self.session_id, b);
        self.write_slice_option(self.passwd.clone(), b);
        self.write_bool(self.read_only, b);
        Ok(())
    }
}
/// ZK 内置的 4 种 scheme
#[derive(Debug, Default)]
pub enum Scheme {
    World,
    IP,
    Digest,
    Super,
}

/// ZooKeeper 权限对象
#[derive(Debug, Default)]
pub struct ACL {
    pub perms: u32,
    pub scheme: Scheme,
    pub id: String,
}

impl Serializer for ACL {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_u32(self.perms, b);
        let scheme = match self.scheme {
            Scheme::World => WORLD,
            Scheme::IP => IP,
            Scheme::Digest => DIGEST,
            Scheme::Super => SUPER,
        };
        self.write_string(scheme, b);
        self.write_string(self.id.as_str(), b);
        Ok(())
    }
}

impl ACL {
    /// world 权限固定写法
    pub fn world_acl() -> Vec<ACL> {
        // TODO 缓存
        vec![ACL {
            perms: Perms::All as u32,
            scheme: Scheme::World,
            id: ANYONE.to_string(),
        }]
    }
}

#[derive(Debug, Default)]
pub(crate) struct CreateRequest {
    path: String,
    data: Option<Vec<u8>>,
    acl: Vec<ACL>,
    flags: i32,
}

impl Serializer for CreateRequest {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_string(self.path.as_str(), b);
        self.write_slice_option(self.data.clone(), b);
        self.write_vec(&self.acl, b);
        self.write_i32(self.flags, b);
        Ok(())
    }
}

impl CreateRequest {
    pub(crate) fn new(path: &str) -> Self {
        CreateRequest {
            path: String::from(path),
            data: None,
            acl: ACL::world_acl(),
            flags: CreateMode::Persistent as i32,
        }
    }

    pub(crate) fn new_full(
        path: String,
        data: Option<&[u8]>,
        acl: Vec<ACL>,
        create_mode: CreateMode,
    ) -> Self {
        let data = match data {
            Some(d) => Some(Vec::from(d)),
            _ => None,
        };
        CreateRequest {
            path,
            data,
            acl,
            flags: create_mode as i32,
        }
    }
}

pub(crate) const DEATH_PTYPE: i8 = -1;

#[derive(Debug)]
pub(crate) struct ReqPacket {
    pub ptype: i8,
    pub rh: Option<RequestHeader>,
    pub req: Option<BytesMut>,
}

impl ReqPacket {
    pub(crate) fn new(rh: Option<RequestHeader>, req: Option<BytesMut>) -> ReqPacket {
        ReqPacket { ptype: 0, rh, req }
    }

    pub(crate) fn death_request() -> ReqPacket {
        ReqPacket {
            ptype: DEATH_PTYPE,
            rh: None,
            req: None,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct DeleteRequest {
    path: String,
    version: i32,
}

impl Serializer for DeleteRequest {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_string(self.path.as_str(), b);
        self.write_i32(self.version, b);
        Ok(())
    }
}
impl DeleteRequest {
    pub(crate) fn new(path: String, version: i32) -> Self {
        DeleteRequest { path, version }
    }
}

#[derive(Debug, Default)]
pub(crate) struct SetDataRequest {
    path: String,
    data: Vec<u8>,
    version: i32,
}

impl Serializer for SetDataRequest {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_string(self.path.as_str(), b);
        self.write_slice(self.data.clone(), b);
        self.write_i32(self.version, b);
        Ok(())
    }
}

impl SetDataRequest {
    pub(crate) fn new(path: String, data: &[u8], version: i32) -> Self {
        SetDataRequest {
            path,
            data: Vec::from(data),
            version,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct GetDataRequest {
    path: String,
    watch: bool,
}

impl Serializer for GetDataRequest {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_string(self.path.as_str(), b);
        self.write_bool(self.watch, b);
        Ok(())
    }
}

impl GetDataRequest {
    pub(crate) fn new(path: String, watch: bool) -> Self {
        GetDataRequest { path, watch }
    }
}
