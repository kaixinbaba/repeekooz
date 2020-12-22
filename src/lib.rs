pub mod protocol;

#[macro_use]
extern crate log;

pub type ZKResult<T> = Result<T, String>;

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

