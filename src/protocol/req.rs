use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

use bytes::BytesMut;

use crate::constants::{AddWatchMode, CreateMode, OpCode, Perms, ANYONE, DIGEST, IP, WORLD};
use crate::protocol::{Deserializer, Serializer};
use crate::{WatcherType, ZKResult};

#[derive(Debug, Default)]
pub(crate) struct RequestHeader {
    pub xid: i32,
    pub rtype: i32,
}

impl RequestHeader {
    pub(crate) fn new(rtype: OpCode) -> RequestHeader {
        RequestHeader {
            xid: 0,
            rtype: rtype.into(),
        }
    }

    pub(crate) fn new_full(xid: i32, rtype: OpCode) -> RequestHeader {
        RequestHeader {
            xid,
            rtype: rtype.into(),
        }
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
/// ZK 内置的 3 种 scheme
/// 第 4 种 Super 其实就是特殊的 Digest
#[derive(Debug, Eq, PartialEq)]
pub enum Scheme {
    World,
    IP(IpAddr),
    // TODO 拆分成加密前的用户名密码，两个字段
    Digest(String),
}

impl From<(String, String)> for Scheme {
    fn from((scheme, id): (String, String)) -> Self {
        match scheme.as_str() {
            WORLD => Scheme::World,
            IP => Scheme::IP(IpAddr::V4(Ipv4Addr::from_str(id.as_str()).unwrap())),
            DIGEST => Scheme::Digest(id),
            _ => panic!("Unknown Scheme : '{}' from server", scheme),
        }
    }
}

/// ZooKeeper 权限对象
/// - `perms`：权限
/// - `scheme`：鉴权模式，详情可见 [`Scheme`]
#[derive(Debug, Eq, PartialEq)]
pub struct ACL {
    // TODO 该字段应该也是枚举对象或者其他有意义的类型，而不是 u32
    pub perms: i32,
    pub scheme: Scheme,
}

impl Serializer for ACL {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_i32(self.perms, b);
        match &self.scheme {
            Scheme::World => {
                self.write_string(WORLD, b);
                self.write_string(ANYONE, b);
            }
            Scheme::IP(addr) => {
                self.write_string(IP, b);
                self.write_string(addr.to_string().as_str(), b);
            }
            Scheme::Digest(digest_info) => {
                self.write_string(DIGEST, b);
                self.write_string(digest_info, b);
            }
        };
        Ok(())
    }
}

impl Deserializer for ACL {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        self.perms = self.read_i32(b);
        let scheme = self.read_string(b);
        let id = self.read_string(b);
        self.scheme = Scheme::from((scheme, id));
        Ok(())
    }
}

impl Default for ACL {
    fn default() -> Self {
        ACL {
            perms: Perms::All.into(),
            scheme: Scheme::World,
        }
    }
}

impl ACL {
    /// world 权限固定写法
    pub fn world_acl() -> Vec<ACL> {
        // TODO 缓存
        vec![ACL::default()]
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
            flags: CreateMode::Persistent.into(),
        }
    }

    pub(crate) fn new_full(
        path: String,
        data: Option<&[u8]>,
        acl: Vec<ACL>,
        create_mode: CreateMode,
    ) -> Self {
        let data = data.map(Vec::from);
        CreateRequest {
            path,
            data,
            acl,
            flags: create_mode.into(),
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
pub(crate) struct PathAndWatchRequest {
    path: String,
    watch: bool,
}

impl Serializer for PathAndWatchRequest {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_string(self.path.as_str(), b);
        self.write_bool(self.watch, b);
        Ok(())
    }
}

impl PathAndWatchRequest {
    pub(crate) fn new(path: String, watch: bool) -> Self {
        PathAndWatchRequest { path, watch }
    }
}

#[derive(Debug, Default)]
pub(crate) struct PathRequest {
    path: String,
}

impl Serializer for PathRequest {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_string(self.path.as_str(), b);
        Ok(())
    }
}

impl PathRequest {
    pub(crate) fn new(path: String) -> Self {
        PathRequest { path }
    }
}

#[derive(Debug, Default)]
pub(crate) struct SetACLRequest {
    path: String,
    acl_list: Vec<ACL>,
    version: i32,
}

impl Serializer for SetACLRequest {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_string(self.path.as_str(), b);
        self.write_vec(&self.acl_list, b);
        self.write_i32(self.version, b);
        Ok(())
    }
}

impl SetACLRequest {
    pub(crate) fn new(path: String, acl_list: Vec<ACL>, version: i32) -> Self {
        SetACLRequest {
            path,
            acl_list,
            version,
        }
    }
}
#[derive(Debug, Default)]
pub(crate) struct AddWatchRequest {
    path: String,
    mode: i32,
}

impl Serializer for AddWatchRequest {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_string(self.path.as_str(), b);
        self.write_i32(self.mode, b);
        Ok(())
    }
}

impl AddWatchRequest {
    pub(crate) fn new(path: String, mode: AddWatchMode) -> Self {
        AddWatchRequest {
            path,
            mode: mode.into(),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct CheckWatchesRequest {
    path: String,
    watcher_type: i32,
}

impl Serializer for CheckWatchesRequest {
    fn write(&self, b: &mut BytesMut) -> ZKResult<()> {
        self.write_string(self.path.as_str(), b);
        self.write_i32(self.watcher_type, b);
        Ok(())
    }
}

impl CheckWatchesRequest {
    pub(crate) fn new(path: String, watcher_type: WatcherType) -> Self {
        CheckWatchesRequest {
            path,
            watcher_type: watcher_type.into(),
        }
    }
}
