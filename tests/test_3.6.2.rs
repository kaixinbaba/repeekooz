#[macro_use]
extern crate log;



use futures_timer::Delay;
use tokio::time::Duration;

use buruma::{AddWatchMode, ACL};
use buruma::{CreateMode, Scheme};
use buruma::{Stat, WatchedEvent, Watcher, ZKResult, ZooKeeper};

const DEFAULT_ZK_SERVER: &str = "127.0.0.1:2181";


#[tokio::test]
async fn full_test() {
    let basic_path = "/buruma";
    let mut zk = ZooKeeper::new(DEFAULT_ZK_SERVER, Duration::from_secs(6))
        .await
        .unwrap();

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
}

