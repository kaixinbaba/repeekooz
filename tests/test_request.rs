#[macro_use]
extern crate log;

use std::thread;

use futures_timer::Delay;
use tokio::time::Duration;

use buruma::{AddWatchMode, ACL};
use buruma::{CreateMode, Scheme};
use buruma::{Stat, WatchedEvent, Watcher, ZKResult, ZooKeeper};

mod common;

#[tokio::test]
async fn basic() {
    let basic_path = "/buruma";
    let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(6))
        .await
        .unwrap();
    // 以防万一先将该节点删除
    zk.delete(basic_path).await;

    // create
    let data = Some("I Love U".as_bytes());
    let path = zk
        .create(basic_path, data, ACL::world_acl(), CreateMode::Persistent)
        .await
        .unwrap();
    assert_eq!(path, basic_path);

    // exists
    let exists_result = zk.exists(basic_path).await.unwrap();
    assert!(exists_result.is_some());
    let not_exists_result = zk.exists("/not_exists_path").await.unwrap();
    assert!(not_exists_result.is_none());

    // set
    let stat = zk.set(basic_path, "buruma".as_bytes()).await.unwrap();
    info!("{:?}", stat);

    // get
    let get_data_result = zk.get(basic_path, None).await.unwrap();
    assert_eq!(
        "buruma".to_string(),
        String::from_utf8(get_data_result).unwrap()
    );

    // get_ephemerals
    for i in 0..3 {
        zk.create(
            format!("/buruma{}", i).as_str(),
            None,
            ACL::world_acl(),
            CreateMode::Ephemeral,
        )
        .await;
    }
    let ephe_vec = zk.get_ephemerals(basic_path).await.unwrap();
    assert_eq!(ephe_vec.len(), 3);

    // children
    let children_list = zk.children(basic_path).await.unwrap();
    assert_eq!(children_list.len(), 0);

    // childrens
    let mut stat = Stat::default();
    let children_list = zk.childrens(basic_path, &mut stat).await.unwrap();
    assert_eq!(children_list.len(), 0);
    assert_eq!(stat.data_length, "buruma".as_bytes().len() as i32);

    // children_count
    let total_count = zk.children_count(basic_path).await.unwrap();
    assert_eq!(total_count, 0);

    // get_acl
    let vec = zk.get_acl(basic_path, None).await.unwrap();
    assert_eq!(vec[0], ACL::default());

    // set_acl
    let acl_list = vec![ACL {
        perms: 21,
        scheme: Scheme::World,
    }];
    zk.set_acl(basic_path, acl_list, -1).await.unwrap();
    let vec = zk.get_acl(basic_path, None).await.unwrap();
    let acl_list = vec![ACL {
        perms: 21,
        scheme: Scheme::World,
    }];
    assert_eq!(vec, acl_list);

    // delete
    zk.delete(basic_path).await.unwrap();
}

#[derive(Debug, Hash)]
struct WatcherDemo;

impl Watcher for WatcherDemo {
    fn process(&self, event: &WatchedEvent) -> ZKResult<()> {
        info!("{:?}", event);
        Ok(())
    }
}

#[tokio::test]
#[ignore]
async fn get_data() {
    let basic_path = "/xjj";
    let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(30))
        .await
        .unwrap();

    let x = zk.getw(basic_path, Some(WatcherDemo), None).await.unwrap();
    info!("first {:?}", String::from_utf8(x));
    Delay::new(Duration::from_secs(10)).await;
}

#[tokio::test]
#[ignore]
async fn exists() {
    let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(10))
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
    let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(30))
        .await
        .unwrap();

    let x = zk.childrenw("/xjj", Some(WatcherDemo)).await;
    info!("{:?}", x);
    Delay::new(Duration::from_secs(10)).await;
}

#[tokio::test]
#[ignore]
async fn childrens() {
    let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(30))
        .await
        .unwrap();

    let mut stat = Stat::default();
    let x = zk.childrens("/xjj", &mut stat).await;
    info!("{:?}, {:?}", x, stat);
}

#[tokio::test]
#[ignore]
async fn children_count() {
    let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(10))
        .await
        .unwrap();

    let x = zk.children_count("/buruma").await;
    info!("{:?}", x);
}

#[tokio::test]
#[ignore]
async fn get_ephemerals() {
    let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(3))
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
    let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(10))
        .await
        .unwrap();

    let x = zk.get_acl("/xjj", None).await;
    info!("{:?}", x);
}

#[tokio::test]
#[ignore]
async fn set_acl() {
    let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(10))
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
    let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(10))
        .await
        .unwrap();

    let x = zk
        .add_watch("/xjj", WatcherDemo, AddWatchMode::PersistentRecursive)
        .await;
    info!("{:?}", x);
    Delay::new(Duration::from_secs(120)).await;
}
