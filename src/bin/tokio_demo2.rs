#[macro_use]
extern crate log;

use std::thread::sleep;
use std::time::Duration;

use tokio::net::TcpStream;
use tokio::prelude::*;

use buruma::constants::Error;
use buruma::ZKResult;
use std::thread;
use buruma::api::ZooKeeper;


#[tokio::main]
async fn main() {
    let zk = ZooKeeper::new("127.0.0.1:2181", 20000).await.unwrap();
    info!("{:?}", zk);
    thread::sleep(Duration::from_secs(10));
    info!("after sleep");
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn new_zk() {
        let zk = ZooKeeper::new("127.0.0.1:2181", 20000).await.unwrap();
        info!("{:?}", zk);
        thread::sleep(Duration::from_secs(10));
        info!("after sleep");
        // let result = zk.create("", None, vec![], CreateMode::Persistent).await;
        // println!("{:?}", result);
    }
}