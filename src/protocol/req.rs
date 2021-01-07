use std::hash::Hasher;

use bytes::BytesMut;

use crate::constants::{CreateMode, Perms, ANYONE, WORLD};
use crate::protocol::Serializer;
use crate::ZKResult;

#[derive(Debug, Default)]
pub struct RequestHeader {
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
pub struct ConnectRequest {
    protocol_version: i32,
    last_zxid_seen: i64,
    time_out: i32,
    session_id: i64,
    passwd: Option<Vec<u8>>,
    read_only: bool,
}

impl ConnectRequest {
    pub(crate) fn new(session_timeout: i32) -> Self {
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
        self.write_i32(self.time_out, b);
        self.write_i64(self.session_id, b);
        self.write_slice_option(self.passwd.clone(), b);
        self.write_bool(self.read_only, b);
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct ACL {
    pub perms: i32,
    pub scheme: String,
    pub id: String,
}

impl Serializer for ACL {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_i32(self.perms, b);
        self.write_string(self.scheme.as_str(), b);
        self.write_string(self.id.as_str(), b);
        Ok(())
    }
}

impl ACL {
    pub fn world_acl() -> Vec<ACL> {
        vec![ACL {
            perms: Perms::All as i32,
            scheme: WORLD.to_string(),
            id: ANYONE.to_string(),
        }]
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

pub const DEATH_PTYPE: i8 = -1;

#[derive(Debug)]
pub struct ReqPacket {
    pub ptype: i8,
    pub rh: Option<RequestHeader>,
    pub req: BytesMut,
}

impl ReqPacket {
    pub(crate) fn new(rh: Option<RequestHeader>, req: BytesMut) -> ReqPacket {
        ReqPacket { ptype: 0, rh, req }
    }

    pub(crate) fn death_request() -> ReqPacket {
        ReqPacket {
            ptype: DEATH_PTYPE,
            rh: None,
            req: BytesMut::with_capacity(0),
        }
    }
}

#[derive(Debug, Default)]
pub struct DeleteRequest {
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
pub struct SetDataRequest {
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
pub struct GetDataRequest {
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
