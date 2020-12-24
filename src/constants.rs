use std::ops::BitOr;

use crate::protocol::req::ACL;

pub enum Perms {
    Read = 1 << 0,
    Write = 1 << 1,
    Create = 1 << 2,
    Delete = 1 << 3,
    ACL = 1 << 4,
    All = Perms::Read as isize | Perms::Write as isize | Perms::Create as isize | Perms::Delete as isize | Perms::ACL as isize,
}

pub const WORLD: &str = "world";
pub const ANYONE: &str = "anyone";

pub enum WatcherType {
    Children = 1,
    Data = 2,
    Any = 3,
}

pub enum EventType {
    None = -1,
    NodeCreated = 1,
    NodeDeleted = 2,
    NodeDataChanged = 3,
    NodeChildrenChanged = 4,
    DataWatchRemoved = 5,
    ChildWatchRemoved = 6,
    PersistentWatchRemoved = 7,
}

pub enum KeeperState {
    Disconnected = 0,
    SyncConnected = 3,
    AuthFailed = 4,
    ConnectedReadOnly = 5,
    SaslAuthenticated = 6,
    Expired = -112,
    Closed = 7,
}

pub enum AddWatchMode {
    Persistent = 0,
    PersistentRecursive = 1,
}

#[derive(Debug, Eq, PartialEq)]
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

pub enum OpCode {
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

pub enum CreateMode {
    Persistent = 0,
    Ephemeral = 1,
    PersistentSequential = 2,
    EphemeralSequential = 3,
    Container = 4,
    PersistentWithTTL = 5,
    PersistentSequentialWithTTL = 6,
}

#[derive(Debug)]
pub enum Error {
    SystemError = -1,
    RuntimeInconsistency = -2,
    DataInconsistency = -3,
    ConnectionLoss = -4,
    MarshallingError = -5,
    Unimplemented = -6,
    OperationTimeout = -7,
    BadArguments = -8,
    NewConfigNoQuorum = -13,
    ReConfigInProgress = -14,
    UnknownSession = -12,
    APIError = -100,
    NoNode = -101,
    NoAuth = -102,
    BadVersion = -103,
    NoChildrenForEphemerals = -108,
    NodeExists = -110,
    NotEmpty = -111,
    SessionExpired = -112,
    InvalidCallback = -113,
    InvalidACL = -114,
    AuthFailed = -115,
    SessionMoved = -118,
    NotReadonly = -119,
    EphemeralOnLocalSession = -120,
    NoWatcher = -121,
    RequestTimeout = -122,
    ReConfigDisabled = -123,
    SessionClosedRequireSASLAuth = -124,
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

