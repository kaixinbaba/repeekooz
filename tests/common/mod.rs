//! 测试的公共模块，存放测试用的公共函数

use cmd_lib::{run_cmd, run_fun};
use tokio::process::Command;
use tokio::time;
use tokio::time::Duration;

use buruma::ZooKeeper;
use std::path::Path;

pub const DEFAULT_ZK_SERVER: &str = "127.0.0.1:2181";

fn jps(name: String) -> Option<u16> {
    let jps_result = run_fun!(jps).unwrap();

    let mut pid_list: Vec<&str> = jps_result
        .split("\n")
        .into_iter()
        .filter(|s| s.contains(&name))
        .collect::<Vec<&str>>();
    if pid_list.is_empty() {
        return None;
    }
    Some(pid_list
        .remove(0)
        .replace(name.as_str(), "").trim()
        .parse::<u16>().unwrap())
}

pub async fn set_up(version: &'static str) {
    let file_path = format!("zkjar/zk{}.jar", version);
    let p = Path::new(&file_path);
    if !p.exists() {
        panic!("The test jar does not exists, path : {}", file_path);
    }
    // 启动测试服务器
    tokio::spawn(async move {
        Command::new("java").arg("-jar").arg(file_path).output().await;
    });

    while let Err(e) = ZooKeeper::new(DEFAULT_ZK_SERVER, Duration::from_secs(6))
        .await {
        // 等待启动
        time::sleep(Duration::from_secs(1)).await;
    }
}


pub fn tear_down(version: &str) {
    let jps_result = run_fun!(jps).unwrap();

    if let Some(pid) = jps(format!("zk{}.jar", version)) {
        run_cmd!(kill -2 $pid);
    }
}




