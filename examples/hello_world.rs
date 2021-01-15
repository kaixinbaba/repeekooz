#[macro_use]
extern crate log;

use std::thread;
use std::time::Duration;

use buruma::api::ZooKeeper;
use buruma::ZKResult;

#[tokio::main]
async fn main() -> ZKResult<()> {
    let client = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(5)).await?;
    info!("{:?}", client);
    Ok(())
}
