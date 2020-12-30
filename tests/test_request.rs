#[macro_use]
extern crate log;


use buruma::api::ZooKeeper;
use buruma::constants::CreateMode;
use buruma::protocol::req::ACL;

mod common;

#[tokio::test]
async fn create() {
    let mut zk = ZooKeeper::new("127.0.0.1:2181", 5000).await.unwrap();
    let data = Some("I Love U".as_bytes());
    let path = zk.create("/xjj", data, ACL::world_acl(), CreateMode::Persistent).await.unwrap();
    assert_eq!(path, "/xjj");
}