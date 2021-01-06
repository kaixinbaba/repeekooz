#[macro_use]
extern crate log;

use buruma::api::ZooKeeper;
use buruma::constants::CreateMode;
use buruma::protocol::req::ACL;

mod common;

#[tokio::test]
async fn basic() {
    let basic_path = "/buruma";
    let mut zk = ZooKeeper::new("127.0.0.1:2181", 5000).await.unwrap();

    // 以防万一先将该节点删除
    zk.delete(basic_path).await;

    // 增加节点
    let data = Some("I Love U".as_bytes());
    let path = zk
        .create(basic_path, data, ACL::world_acl(), CreateMode::Persistent)
        .await
        .unwrap();
    assert_eq!(path, basic_path);
    // 删除节点
    zk.delete(basic_path).await.unwrap();
}
