use std::fmt::{Display, Formatter};
use std::io::Error;

use cmd_lib::log::SetLoggerError;
use thiserror::Error;

#[derive(Debug)]
pub enum ServerInfo {
    Host,
    Ip,
    Port,
    Chroot,
}

impl Display for ServerInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ServerInfo::Host => "Host",
            ServerInfo::Ip => "Ip address",
            ServerInfo::Port => "Port",
            ServerInfo::Chroot => "Chroot",
        })
    }
}



#[derive(Error, Debug)]
pub enum ZKError {
    /// ArgumentError
    #[error("The `{0}` is not correct [{1}]")]
    ServerInfoError(ServerInfo, String),
    #[error("The path `{0}` that you provide is not legitimate [{1}]")]
    PathError(String, String),
    #[error("NetworkError detail : {0}")]
    NetworkError(String),

    #[error("Parse protocol occur error")]
    ProtocolParseError,

    #[error("UnknownError")]
    UnknownError,

    #[error("Received error from ZooKeeper server message is {0}, error code is {1}")]
    ServerError(ServerErrorCode, i32),
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for ZKError {
    fn from(e: tokio::sync::mpsc::error::SendError<T>) -> Self {
        ZKError::NetworkError(e.to_string())
    }
}

impl From<log::SetLoggerError> for ZKError {
    fn from(_: SetLoggerError) -> Self {
        ZKError::UnknownError
    }
}

impl From<std::io::Error> for ZKError {
    fn from(_: Error) -> Self {
        ZKError::UnknownError
    }
}


/// buruma 常见错误
#[derive(Error, Debug, Eq, PartialEq)]
pub enum ServerErrorCode {
    SystemError,
    RuntimeInconsistency,
    DataInconsistency,
    ConnectionLoss,
    MarshallingError,
    Unimplemented,
    OperationTimeout,
    BadArguments,
    UnknownSession,
    NewConfigNoQuorum,
    ReConfigInProgress,
    APIError,
    NoNode,
    NoAuth,
    BadVersion,
    NoChildrenForEphemerals,
    NodeExists,
    NotEmpty,
    SessionExpired,
    InvalidCallback,
    InvalidACL,
    AuthFailed,
    SessionMoved,
    NotReadonly,
    EphemeralOnLocalSession,
    NoWatcher,
    RequestTimeout,
    ReConfigDisabled,
    SessionClosedRequireSASLAuth,
}


impl Display for ServerErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ServerErrorCode::SystemError => "SystemError",
            ServerErrorCode::RuntimeInconsistency => "RuntimeInconsistency",
            ServerErrorCode::DataInconsistency => "DataInconsistency",
            ServerErrorCode::ConnectionLoss => "ConnectionLoss",
            ServerErrorCode::MarshallingError => "MarshallingError",
            ServerErrorCode::Unimplemented => "Unimplemented",
            ServerErrorCode::OperationTimeout => "OperationTimeout",
            ServerErrorCode::BadArguments => "BadArguments",
            ServerErrorCode::UnknownSession => "UnknownSession",
            ServerErrorCode::NewConfigNoQuorum => "NewConfigNoQuorum",
            ServerErrorCode::ReConfigInProgress => "ReConfigInProgress",
            ServerErrorCode::APIError => "APIError",
            ServerErrorCode::NoNode => "NoNode",
            ServerErrorCode::NoAuth => "NoAuth",
            ServerErrorCode::BadVersion => "BadVersion",
            ServerErrorCode::NoChildrenForEphemerals => "NoChildrenForEphemerals",
            ServerErrorCode::NodeExists => "NodeExists",
            ServerErrorCode::NotEmpty => "NotEmpty",
            ServerErrorCode::SessionExpired => "SessionExpired",
            ServerErrorCode::InvalidCallback => "InvalidCallback",
            ServerErrorCode::InvalidACL => "InvalidACL",
            ServerErrorCode::AuthFailed => "AuthFailed",
            ServerErrorCode::SessionMoved => "SessionMoved",
            ServerErrorCode::NotReadonly => "NotReadonly",
            ServerErrorCode::EphemeralOnLocalSession => "EphemeralOnLocalSession",
            ServerErrorCode::NoWatcher => "NoWatcher",
            ServerErrorCode::RequestTimeout => "RequestTimeout",
            ServerErrorCode::ReConfigDisabled => "ReConfigDisabled",
            ServerErrorCode::SessionClosedRequireSASLAuth => "SessionClosedRequireSASLAuth",
        })
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


#[cfg(test)]
mod test {
    use crate::ZKError;

    #[test]
    fn test_error() {
    }
}