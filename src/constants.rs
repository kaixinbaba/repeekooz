#![allow(unused)]

use std::fmt::{Display, Formatter};

/// ZooKeeper 定义的 5 种权限
/// `Read`： 节点可读        0 0 0 0 1
/// `Write`： 节点可写       0 0 0 1 0
/// `Create`： 可创建子节点   0 0 1 0 0
/// `Delete`： 可删除子节点   0 1 0 0 0
/// `ACL`： 可读写 ACL 数据  1 0 0 0 0
/// `All`： 所有以上权限     1 1 1 1 1
#[derive(Debug)]
pub enum Perms {
    Read,
    Write,
    Create,
    Delete,
    ACL,
    All,
}

impl From<Perms> for i32 {
    fn from(perms: Perms) -> Self {
        match perms {
            Perms::Read => 1,
            Perms::Write => 2,
            Perms::Create => 4,
            Perms::Delete => 8,
            Perms::ACL => 16,
            Perms::All => 31,
        }
    }
}

impl Display for Perms {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Perms::Read => "R",
            Perms::Write => "W",
            Perms::Create => "C",
            Perms::Delete => "D",
            Perms::ACL => "A",
            Perms::All => "RWCDA",
        };
        f.write_str(s);
        Ok(())
    }
}

/// world scheme
pub const WORLD: &str = "world";
/// ip scheme
pub const IP: &str = "ip";
/// digest scheme
pub const DIGEST: &str = "digest";
/// super scheme
pub const SUPER: &str = "super";
/// World scheme 固定的 id
pub const ANYONE: &str = "anyone";
/// 忽略版本号，一般用于 set_data 或 delete
pub const IGNORE_VERSION: i32 = -1;

pub enum VersionType {
    Version(i32),
    NoVersion,
}

impl From<VersionType> for i32 {
    fn from(version_type: VersionType) -> Self {
        match version_type {
            VersionType::Version(v) => v,
            VersionType::NoVersion => IGNORE_VERSION,
        }
    }
}

pub enum WatcherType {
    Children,
    Data,
    Any,
}

impl From<WatcherType> for i32 {
    fn from(watch_type: WatcherType) -> Self {
        match watch_type {
            WatcherType::Children => 1,
            WatcherType::Data => 2,
            WatcherType::Any => 3,
        }
    }
}

/// 回调的事件类型
#[derive(Debug)]
pub enum EventType {
    None,
    /// 节点创建
    NodeCreated,
    /// 节点删除
    NodeDeleted,
    /// 节点数据变更
    NodeDataChanged,
    /// 节点子节点列表变更
    NodeChildrenChanged,
    /// 节点数据的 Watcher 监听被移除
    DataWatchRemoved,
    /// 节点子节点的 Watcher 监听被移除
    ChildWatchRemoved,
    /// 持久的 Watcher 监听被移除
    PersistentWatchRemoved,
}

impl From<EventType> for isize {
    fn from(event_type: EventType) -> Self {
        match event_type {
            EventType::None => -1,
            EventType::NodeCreated => 1,
            EventType::NodeDeleted => 2,
            EventType::NodeDataChanged => 3,
            EventType::NodeChildrenChanged => 4,
            EventType::DataWatchRemoved => 5,
            EventType::ChildWatchRemoved => 6,
            EventType::PersistentWatchRemoved => 7,
        }
    }
}

impl From<i32> for EventType {
    fn from(code: i32) -> Self {
        EventType::from(code as isize)
    }
}

impl From<isize> for EventType {
    fn from(code: isize) -> Self {
        match code {
            -1 => EventType::None,
            1 => EventType::NodeCreated,
            2 => EventType::NodeDeleted,
            3 => EventType::NodeDataChanged,
            4 => EventType::NodeChildrenChanged,
            5 => EventType::DataWatchRemoved,
            6 => EventType::ChildWatchRemoved,
            7 => EventType::PersistentWatchRemoved,
            _ => panic!("Invalid code [{}] for EventType", code),
        }
    }
}

/// 服务端的状态
#[derive(Debug, Eq, PartialEq)]
pub enum KeeperState {
    /// 未连接
    Disconnected,
    /// 同步完成
    SyncConnected,
    /// 鉴权失败
    AuthFailed,
    /// 以只读状态连接
    ConnectedReadOnly,
    /// SASL 验证通过
    SaslAuthenticated,
    /// 会话过期
    Expired,
    /// 关闭
    Closed,
}

impl From<KeeperState> for isize {
    fn from(keeper_state: KeeperState) -> Self {
        match keeper_state {
            KeeperState::Disconnected => 0,
            KeeperState::SyncConnected => 3,
            KeeperState::AuthFailed => 4,
            KeeperState::ConnectedReadOnly => 5,
            KeeperState::SaslAuthenticated => 6,
            KeeperState::Expired => -112,
            KeeperState::Closed => 7,
        }
    }
}

impl From<i32> for KeeperState {
    fn from(code: i32) -> Self {
        KeeperState::from(code as isize)
    }
}

impl From<isize> for KeeperState {
    fn from(code: isize) -> Self {
        match code {
            0 => KeeperState::Disconnected,
            3 => KeeperState::SyncConnected,
            4 => KeeperState::AuthFailed,
            5 => KeeperState::ConnectedReadOnly,
            6 => KeeperState::SaslAuthenticated,
            -112 => KeeperState::Expired,
            7 => KeeperState::Closed,
            _ => panic!("Invalid code [{}] for KeeperState", code),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AddWatchMode {
    Persistent,
    PersistentRecursive,
}

impl From<AddWatchMode> for i32 {
    fn from(add_watch_mode: AddWatchMode) -> Self {
        match add_watch_mode {
            AddWatchMode::Persistent => 0,
            AddWatchMode::PersistentRecursive => 1,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum States {
    Connecting,
    Associating,
    Connected,
    ConnectedReadOnly,
    Closed,
    AuthFailed,
    NotConnected,
}

impl States {
    pub fn is_alive(&self) -> bool {
        self.ne(&States::Closed) && self.ne(&States::AuthFailed)
    }

    pub fn is_connected(&self) -> bool {
        self.eq(&States::Connected) || self.eq(&States::ConnectedReadOnly)
    }
}

pub(crate) enum OpCode {
    Notification,
    Create,
    Delete,
    Exists,
    GetData,
    SetData,
    GetACL,
    SetACL,
    GetChildren,
    Sync,
    Ping,
    GetChildren2,
    Check,
    Multi,
    Create2,
    ReConfig,
    CheckWatches,
    RemoveWatches,
    CreateContainer,
    DeleteContainer,
    CreateTTL,
    MultiRead,
    Auth,
    SetWatches,
    Sasl,
    GetEphemerals,
    GetAllChildrenNumber,
    SetWatches2,
    AddWatch,
    CreateSession,
    CloseSession,
    Error,
}

impl From<OpCode> for i32 {
    fn from(code: OpCode) -> Self {
        match code {
            OpCode::Notification => 0,
            OpCode::Create => 1,
            OpCode::Delete => 2,
            OpCode::Exists => 3,
            OpCode::GetData => 4,
            OpCode::SetData => 5,
            OpCode::GetACL => 6,
            OpCode::SetACL => 7,
            OpCode::GetChildren => 8,
            OpCode::Sync => 9,
            OpCode::Ping => 11,
            OpCode::GetChildren2 => 12,
            OpCode::Check => 13,
            OpCode::Multi => 14,
            OpCode::Create2 => 15,
            OpCode::ReConfig => 16,
            OpCode::CheckWatches => 17,
            OpCode::RemoveWatches => 18,
            OpCode::CreateContainer => 19,
            OpCode::DeleteContainer => 20,
            OpCode::CreateTTL => 21,
            OpCode::MultiRead => 22,
            OpCode::Auth => 100,
            OpCode::SetWatches => 101,
            OpCode::Sasl => 102,
            OpCode::GetEphemerals => 103,
            OpCode::GetAllChildrenNumber => 104,
            OpCode::SetWatches2 => 105,
            OpCode::AddWatch => 106,
            OpCode::CreateSession => -10,
            OpCode::CloseSession => -11,
            OpCode::Error => -1,
        }
    }
}

pub enum OpKind {
    Transaction,
    Read,
}

/// 创建的节点类型
#[derive(Debug, Eq, PartialEq)]
pub enum CreateMode {
    /// 持久节点
    Persistent,
    /// 临时节点，生命周期同客户端会话
    Ephemeral,
    /// 持久顺序节点，自动添加自增序号后缀
    PersistentSequential,
    /// 临时顺序节点，自动添加自增序号后缀
    EphemeralSequential,
    /// 容器节点
    Container,
    /// 带超时时间的持久节点
    PersistentWithTTL,
    /// 带超时时间的持久顺序节点
    PersistentSequentialWithTTL,
}

impl CreateMode {
    pub fn is_container(&self) -> bool {
        self.eq(&CreateMode::Container)
    }
}

impl From<CreateMode> for i32 {
    fn from(create_mode: CreateMode) -> Self {
        isize::from(create_mode) as i32
    }
}
impl From<CreateMode> for isize {
    fn from(create_mode: CreateMode) -> Self {
        match create_mode {
            CreateMode::Persistent => 0,
            CreateMode::Ephemeral => 1,
            CreateMode::PersistentSequential => 2,
            CreateMode::EphemeralSequential => 3,
            CreateMode::Container => 4,
            CreateMode::PersistentWithTTL => 5,
            CreateMode::PersistentSequentialWithTTL => 6,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum XidType {
    Notification,
    Ping,
    AuthPacket,
    SetWatches,
    Response,
}

impl From<XidType> for i32 {
    fn from(xid_type: XidType) -> Self {
        match xid_type {
            XidType::Notification => -1,
            XidType::Ping => -2,
            XidType::AuthPacket => -4,
            XidType::SetWatches => -8,
            XidType::Response => 0,
        }
    }
}

impl From<i32> for XidType {
    fn from(code: i32) -> Self {
        XidType::from(code as isize)
    }
}

impl From<isize> for XidType {
    fn from(code: isize) -> Self {
        match code {
            -1 => XidType::Notification,
            -2 => XidType::Ping,
            -4 => XidType::AuthPacket,
            -8 => XidType::SetWatches,
            _ => XidType::Response,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_as() {
        let code: i32 = WatcherType::Any.into();
        println!("{}", code);
    }
}
