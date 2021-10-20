use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {

    #[error("The argument `{0}` is not legitimate, error message is {1}")]
    ArgumentError(String, String),

    #[error("Parse protocol occur error")]
    ProtocolParseError,

    #[error("There is something wrong with {0}")]
    InternalError(String),

    #[error("Received error from ZooKeeper server message is {0}, error code is {1}")]
    ServerError(String, isize),

}