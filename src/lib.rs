pub mod protocol;
pub mod constants;
pub mod api;
mod client;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

use crate::constants::Error;

pub struct ZKError(Error, &'static str);

pub type ZKResult<T> = Result<T, ZKError>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        pretty_env_logger::init();
        debug!("test bugggg");
        info!("test such information");
        warn!("test o_O");
        error!("test error");
    }
}

