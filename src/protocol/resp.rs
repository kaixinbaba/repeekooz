use bytes::BytesMut;

use crate::protocol::Deserializer;
use crate::{ZKResult, ACL};

#[derive(Debug, Default)]
pub(crate) struct ReplyHeader {
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
pub(crate) struct ConnectResponse {
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
pub(crate) struct CreateResponse {
    pub path: String,
}

impl Deserializer for CreateResponse {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        self.path = self.read_string(b);
        Ok(())
    }
}

#[derive(Debug, Default)]
pub(crate) struct IgnoreResponse {}

impl Deserializer for IgnoreResponse {
    fn read(&mut self, _b: &mut BytesMut) -> ZKResult<()> {
        Ok(())
    }
}

/// ZK 节点统计数据
/// - `czxid`： 创建节点时 zxid
/// - `mzxid`： 修改节点时 zxid
/// - `ctime`： 创建时间戳
/// - `mtime`： 修改时间戳
/// - `version`： 节点数据修改次数
/// - `cversion`： 子节点列表修改次数
/// - `aversion`： 节点 ACL 数据修改次数
/// - `ephemeral_owner`：若当前节点是临时节点，该字段为对应客户端的 session_id，否则为 0
/// - `data_length`： 数据的长度
/// - `num_children`：子节点（不含孙子节点）数量
/// - `pzxid`：
#[derive(Debug, Default)]
pub struct Stat {
    pub czxid: i64,
    pub mzxid: i64,
    pub ctime: i64,
    pub mtime: i64,
    pub version: i32,
    pub cversion: i32,
    pub aversion: i32,
    pub ephemeral_owner: i64,
    pub data_length: i32,
    pub num_children: i32,
    pub pzxid: i64,
}

impl Deserializer for Stat {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        self.czxid = self.read_i64(b);
        self.mzxid = self.read_i64(b);
        self.ctime = self.read_i64(b);
        self.mtime = self.read_i64(b);
        self.version = self.read_i32(b);
        self.cversion = self.read_i32(b);
        self.aversion = self.read_i32(b);
        self.ephemeral_owner = self.read_i64(b);
        self.data_length = self.read_i32(b);
        self.num_children = self.read_i32(b);
        self.pzxid = self.read_i64(b);
        Ok(())
    }
}

#[derive(Debug, Default)]
pub(crate) struct SetDataResponse {
    pub stat: Stat,
}

impl Deserializer for SetDataResponse {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        self.stat.read(b)?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub(crate) struct GetDataResponse {
    pub data: Vec<u8>,
    pub stat: Stat,
}

impl Deserializer for GetDataResponse {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        self.data = self.read_slice_unchecked(b);
        self.stat.read(b)?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub(crate) struct PathListResponse {
    pub path_list: Vec<String>,
}

impl Deserializer for PathListResponse {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        let len = self.read_i32(b);
        if len != -1 {
            for _ in 0..len {
                let path = self.read_string(b);
                self.path_list.push(path);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
pub(crate) struct GetChildren2Response {
    pub path_list: Vec<String>,
    pub stat: Stat,
}

impl Deserializer for GetChildren2Response {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        let len = self.read_i32(b);
        if len != -1 {
            for _ in 0..len {
                let path = self.read_string(b);
                self.path_list.push(path);
            }
        }
        self.stat.read(b)?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub(crate) struct GetACLResponse {
    pub acl_list: Vec<ACL>,
    pub stat: Stat,
}

impl Deserializer for GetACLResponse {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        let len = self.read_i32(b);
        if len != -1 {
            for _ in 0..len {
                let mut acl = ACL::default();
                acl.read(b)?;
                self.acl_list.push(acl);
            }
        }
        self.stat.read(b)?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub(crate) struct GetAllChildrenNumberResponse {
    pub total_number: u32,
}

impl Deserializer for GetAllChildrenNumberResponse {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        self.total_number = self.read_u32(b);
        Ok(())
    }
}

#[derive(Debug, Default)]
pub(crate) struct DummyResponse;

impl Deserializer for DummyResponse {
    fn read(&mut self, _b: &mut BytesMut) -> ZKResult<()> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub(crate) struct WatcherEvent {
    pub keep_state: i32,
    pub event_type: i32,
    pub path: String,
}

impl Deserializer for WatcherEvent {
    fn read(&mut self, b: &mut BytesMut) -> ZKResult<()> {
        self.event_type = self.read_i32(b);
        self.keep_state = self.read_i32(b);
        self.path = self.read_string(b);
        Ok(())
    }
}
