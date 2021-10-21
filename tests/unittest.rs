#![allow(unused)]
#[macro_use]
extern crate log;


use futures_timer::Delay;
use tokio::time::Duration;

use buruma::{AddWatchMode, ACL};
use buruma::{CreateMode, Scheme};
use buruma::{Stat, WatchedEvent, Watcher, ZKResult, ZooKeeper};


const DEFAULT_ZK_SERVER: &str = "127.0.0.1:2181";

#[derive(Debug, Hash)]
struct WatcherDemo;

impl Watcher for WatcherDemo {
    fn process(&self, event: &WatchedEvent) {
        info!("{:?}", event);
    }
}

#[tokio::test]
#[ignore]
async fn get_data() {
    let basic_path = "/xjj";
    let mut zk = ZooKeeper::new(DEFAULT_ZK_SERVER, Duration::from_secs(30))
        .await
        .unwrap();

    let x = zk.getw(basic_path, Some(WatcherDemo), None).await.unwrap();
    info!("first {:?}", String::from_utf8(x));
    Delay::new(Duration::from_secs(10)).await;
}

#[tokio::test]
#[ignore]
async fn exists() {
    let mut zk = ZooKeeper::new(DEFAULT_ZK_SERVER, Duration::from_secs(10))
        .await
        .unwrap();

    let x = zk.exists("/xjj").await;
    info!("{:?}", x);
    let x = zk.existsw("/xjj", Some(WatcherDemo)).await;
    info!("{:?}", x);
    Delay::new(Duration::from_secs(5)).await;
    let x = zk.exists("/notExists").await;
    info!("{:?}", x);
}

#[tokio::test]
#[ignore]
async fn children() {
    let mut zk = ZooKeeper::new(DEFAULT_ZK_SERVER, Duration::from_secs(30))
        .await
        .unwrap();

    let x = zk.childrenw("/xjj", Some(WatcherDemo)).await;
    info!("{:?}", x);
    Delay::new(Duration::from_secs(10)).await;
}

#[tokio::test]
#[ignore]
async fn childrens() {
    let mut zk = ZooKeeper::new(DEFAULT_ZK_SERVER, Duration::from_secs(30))
        .await
        .unwrap();

    let mut stat = Stat::default();
    let x = zk.childrens("/xjj", &mut stat).await;
    info!("{:?}, {:?}", x, stat);
}

#[tokio::test]
#[ignore]
async fn children_count() {
    let mut zk = ZooKeeper::new(DEFAULT_ZK_SERVER, Duration::from_secs(10))
        .await
        .unwrap();

    let x = zk.children_count("/buruma").await;
    info!("{:?}", x);
}

#[tokio::test]
#[ignore]
async fn get_ephemerals() {
    let mut zk = ZooKeeper::new(DEFAULT_ZK_SERVER, Duration::from_secs(3))
        .await
        .unwrap();
    // setup
    let result = zk.delete("/testppp/test11").await;
    info!("{:?}", result);

    let result1 = zk.delete("/testppp").await;
    info!("{:?}", result1);

    let data = Some("".as_bytes());
    for i in 0..3 {
        zk.create(
            format!("/test{}", i).as_str(),
            None,
            ACL::world_acl(),
            CreateMode::Ephemeral,
        )
            .await;
    }
    zk.create("/testppp", data, ACL::world_acl(), CreateMode::Persistent)
        .await;
    zk.create(
        "/testppp/test11",
        data,
        ACL::world_acl(),
        CreateMode::Ephemeral,
    )
        .await;

    let x = zk.get_ephemerals("/test").await.unwrap();
    assert_eq!(x.len(), 4);
}

#[tokio::test]
#[ignore]
async fn get_acl() {
    let mut zk = ZooKeeper::new(DEFAULT_ZK_SERVER, Duration::from_secs(10))
        .await
        .unwrap();

    let x = zk.get_acl("/xjj", None).await;
    info!("{:?}", x);
}

#[tokio::test]
#[ignore]
async fn set_acl() {
    let mut zk = ZooKeeper::new(DEFAULT_ZK_SERVER, Duration::from_secs(10))
        .await
        .unwrap();

    let acl_list = vec![ACL {
        perms: 21,
        scheme: Scheme::World,
    }];
    let x = zk.set_acl("/xjj", acl_list, -1).await;
    info!("{:?}", x);
}

#[tokio::test]
#[ignore]
async fn add_watch() {
    let mut zk = ZooKeeper::new(DEFAULT_ZK_SERVER, Duration::from_secs(10))
        .await
        .unwrap();

    let x = zk
        .add_watch("/xjj", WatcherDemo, AddWatchMode::PersistentRecursive)
        .await;
    info!("{:?}", x);
    Delay::new(Duration::from_secs(120)).await;
}


#[test]
fn test_from_string() {
    println!("hello")
}
