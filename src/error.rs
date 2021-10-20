use thiserror::Error;
use std::fmt::{Display, Formatter};

#[derive(Error, Debug)]
pub enum ZKError {

    #[error("The argument `{0}` is not legitimate, error message is {1}")]
    ArgumentError(String, String),

    #[error("There is something wrong with network")]
    NetworkError,

    #[error("Parse protocol occur error")]
    ProtocolParseError,

    #[error("UnknownError")]
    UnknownError,

    #[error("Received error from ZooKeeper server message is {0}, error code is {1}")]
    ServerError(ServerErrorCode, i32),

}

/// buruma 常见错误
#[derive(Error, Debug, Eq, PartialEq)]
pub enum ServerErrorCode {
    SystemError = -1,
    RuntimeInconsistency = -2,
    DataInconsistency = -3,
    ConnectionLoss = -4,
    MarshallingError = -5,
    Unimplemented = -6,
    OperationTimeout = -7,
    BadArguments = -8,
    UnknownSession = -12,
    NewConfigNoQuorum = -13,
    ReConfigInProgress = -14,
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

impl Display for ServerErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}

impl From<i32> for ServerErrorCode {
    fn from(code: i32) -> Self {
        Self::from(code as isize)
    }
}

impl From<isize> for ServerErrorCode {
    fn from(code: isize) -> Self {
        match code {
            -1 => ServerErrorCode::SystemError,
            -2 => ServerErrorCode::RuntimeInconsistency,
            -3 => ServerErrorCode::DataInconsistency,
            -4 => ServerErrorCode::ConnectionLoss,
            -5 => ServerErrorCode::MarshallingError,
            -6 => ServerErrorCode::Unimplemented,
            -7 => ServerErrorCode::OperationTimeout,
            -8 => ServerErrorCode::BadArguments,
            -12 => ServerErrorCode::UnknownSession,
            -13 => ServerErrorCode::NewConfigNoQuorum,
            -14 => ServerErrorCode::ReConfigInProgress,
            -100 => ServerErrorCode::APIError,
            -101 => ServerErrorCode::NoNode,
            -102 => ServerErrorCode::NoAuth,
            -103 => ServerErrorCode::BadVersion,
            -108 => ServerErrorCode::NoChildrenForEphemerals,
            -110 => ServerErrorCode::NodeExists,
            -111 => ServerErrorCode::NotEmpty,
            -112 => ServerErrorCode::SessionExpired,
            -113 => ServerErrorCode::InvalidCallback,
            -114 => ServerErrorCode::InvalidACL,
            -115 => ServerErrorCode::AuthFailed,
            -118 => ServerErrorCode::SessionMoved,
            -119 => ServerErrorCode::NotReadonly,
            -120 => ServerErrorCode::EphemeralOnLocalSession,
            -121 => ServerErrorCode::NoWatcher,
            -122 => ServerErrorCode::RequestTimeout,
            -123 => ServerErrorCode::ReConfigDisabled,
            -124 => ServerErrorCode::SessionClosedRequireSASLAuth,
            _ => ServerErrorCode::SystemError,
        }
    }
}
