#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use crate::constants::Error;

pub mod api;
mod client;
pub mod constants;
pub mod protocol;

#[derive(Debug)]
pub struct ZKError(Error, &'static str);

pub type ZKResult<T> = Result<T, ZKError>;
