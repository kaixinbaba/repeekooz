#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use crate::constants::Error;

pub mod protocol;
pub mod constants;
pub mod api;
mod client;

#[derive(Debug)]
pub struct ZKError(Error, &'static str);

pub type ZKResult<T> = Result<T, ZKError>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        pretty_env_logger::init();
        debug!("test bugggg");
        info!("test such information");
        warn!("test o_O");
        error!("test error");
    }
}

