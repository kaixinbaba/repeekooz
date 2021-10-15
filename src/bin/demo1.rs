use tokio::process::Command;

#[tokio::main]
async fn main() -> () {
    // 启动测试服务器
    tokio::spawn(async {
        Command::new("java").arg("-jar").arg("zkjar/zk36.jar").output().await;
    });
}
