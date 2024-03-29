#![allow(unused)]
#[macro_use]
extern crate log;

use futures_timer::Delay;
use tokio::time::Duration;

use repeekooz::{AddWatchMode, ACL};
use repeekooz::{CreateMode, Scheme};
use repeekooz::{Stat, WatchedEvent, Watcher, ZKResult, ZooKeeper};

const DEFAULT_ZK_SERVER: &str = "127.0.0.1:2181";

#[tokio::test]
#[ignore]
async fn full_test() {
    let basic_path = "/repeekooz363";
    let mut zk = ZooKeeper::new(DEFAULT_ZK_SERVER, Duration::from_secs(6))
        .await
        .unwrap();

    // zero
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
    let stat = zk.set(basic_path, "repeekooz".as_bytes()).await.unwrap();
    info!("{:?}", stat);

    // get
    let get_data_result = zk.get(basic_path, None).await.unwrap();
    assert_eq!(
        "repeekooz".to_string(),
        String::from_utf8(get_data_result).unwrap()
    );

    // get_ephemerals
    for i in 0..3 {
        zk.create(
            format!("{}/{}", basic_path, i).as_str(),
            None,
            ACL::world_acl(),
            CreateMode::Ephemeral,
        )
        .await
        .unwrap();
    }
    let ephe_vec = zk.get_ephemerals(basic_path).await.unwrap();
    assert_eq!(ephe_vec.len(), 3);

    // children
    let children_list = zk.children(basic_path).await.unwrap();
    assert_eq!(children_list.len(), 3);

    // childrens
    let mut stat = Stat::default();
    let children_list = zk.childrens(basic_path, &mut stat).await.unwrap();
    assert_eq!(children_list.len(), 3);
    assert_eq!(stat.data_length, "repeekooz".as_bytes().len() as i32);

    // children_count
    let total_count = zk.children_count(basic_path).await.unwrap();
    assert_eq!(total_count, 3);

    // get_acl
    let vec = zk.get_acl(basic_path, None).await.unwrap();
    assert_eq!(vec[0], ACL::default());

    // set_acl
    let acl_list = vec![ACL {
        perms: 15,
        scheme: Scheme::World,
    }];
    zk.set_acl(basic_path, acl_list, -1).await.unwrap();
    let vec = zk.get_acl(basic_path, None).await.unwrap();
    let acl_list = vec![ACL {
        perms: 15,
        scheme: Scheme::World,
    }];
    assert_eq!(vec, acl_list);
    for i in 0..3 {
        zk.delete(format!("{}/{}", basic_path, i).as_str())
            .await
            .unwrap();
    }
    zk.delete(basic_path).await.unwrap();
}
