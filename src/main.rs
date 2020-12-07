#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init();
    debug!("bugggg");
    info!("such information");
    warn!("o_O");
    error!("error");
}