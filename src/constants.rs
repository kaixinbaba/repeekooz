/// ZooKeeper 定义的 5 种权限
/// `Read`： 节点可读
/// `Write`： 节点可写
/// `Create`： 可创建子节点
/// `Delete`： 可删除子节点
/// `ACL`： 可读写 ACL 数据
/// `All`： 所有以上权限
pub enum Perms {
    Read = 1 << 0,
    Write = 1 << 1,
    Create = 1 << 2,
    Delete = 1 << 3,
    ACL = 1 << 4,
    All = Perms::Read as isize
        | Perms::Write as isize
        | Perms::Create as isize
        | Perms::Delete as isize
        | Perms::ACL as isize,
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

pub enum WatcherType {
    Children = 1,
    Data = 2,
    Any = 3,
}
/// 回调的事件类型
#[derive(Debug)]
pub enum EventType {
    None = -1,
    /// 节点创建
    NodeCreated = 1,
    /// 节点删除
    NodeDeleted = 2,
    /// 节点数据变更
    NodeDataChanged = 3,
    /// 节点子节点列表变更
    NodeChildrenChanged = 4,
    /// 节点数据的 Watcher 监听被移除
    DataWatchRemoved = 5,
    /// 节点子节点的 Watcher 监听被移除
    ChildWatchRemoved = 6,
    /// 持久的 Watcher 监听被移除
    PersistentWatchRemoved = 7,
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
    Disconnected = 0,
    /// 同步完成
    SyncConnected = 3,
    /// 鉴权失败
    AuthFailed = 4,
    /// 以只读状态连接
    ConnectedReadOnly = 5,
    /// SASL 验证通过
    SaslAuthenticated = 6,
    /// 会话过期
    Expired = -112,
    /// 关闭
    Closed = 7,
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
    Persistent = 0,
    PersistentRecursive = 1,
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
    Notification = 0,
    Create = 1,
    Delete = 2,
    Exists = 3,
    GetData = 4,
    SetData = 5,
    GetACL = 6,
    SetACL = 7,
    GetChildren = 8,
    Sync = 9,
    Ping = 11,
    GetChildren2 = 12,
    Check = 13,
    Multi = 14,
    Create2 = 15,
    ReConfig = 16,
    CheckWatches = 17,
    RemoveWatches = 18,
    CreateContainer = 19,
    DeleteContainer = 20,
    CreateTTL = 21,
    MultiRead = 22,
    Auth = 100,
    SetWatches = 101,
    Sasl = 102,
    GetEphemerals = 103,
    GetAllChildrenNumber = 104,
    SetWatches2 = 105,
    AddWatch = 106,
    CreateSession = -10,
    CloseSession = -11,
    Error = -1,
}

pub enum OpKind {
    Transaction,
    Read,
}

/// 创建的节点类型
#[derive(Debug, Eq, PartialEq)]
pub enum CreateMode {
    /// 持久节点
    Persistent = 0,
    /// 临时节点，生命周期同客户端会话
    Ephemeral = 1,
    /// 持久顺序节点，自动添加自增序号后缀
    PersistentSequential = 2,
    /// 临时顺序节点，自动添加自增序号后缀
    EphemeralSequential = 3,
    /// 容器节点
    Container = 4,
    /// 带超时时间的持久节点
    PersistentWithTTL = 5,
    /// 带超时时间的持久顺序节点
    PersistentSequentialWithTTL = 6,
}

impl CreateMode {
    pub fn is_container(&self) -> bool {
        self.eq(&CreateMode::Container)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum XidType {
    Notification = -1,
    Ping = -2,
    AuthPacket = -4,
    SetWatches = -8,
    Response,
}

impl From<i32> for XidType {
    fn from(code: i32) -> Self {
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
    fn test_static_constant() {
        assert_eq!("world", WORLD);
        assert_eq!("anyone", ANYONE);
    }

    #[test]
    fn test_perms() {
        assert_eq!(Perms::Read as isize, 1);
        assert_eq!(Perms::Write as isize, 2);
        assert_eq!(Perms::Create as isize, 4);
        assert_eq!(Perms::Delete as isize, 8);
        assert_eq!(Perms::ACL as isize, 16);
        assert_eq!(Perms::All as isize, 31);
    }
}
