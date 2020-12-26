use crate::client::Client;
use crate::ZKResult;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use tokio::time::Duration;
use crate::protocol::req::ACL;
use crate::constants::CreateMode;

#[derive(Debug)]
pub struct ZooKeeper {
    client: Client,
}

impl ZooKeeper {
    pub async fn new(connect_string: &str, session_timeout: i32) -> ZKResult<ZooKeeper> {
        pretty_env_logger::init();
        let client = Client::new(connect_string, session_timeout).await?;
        Ok(ZooKeeper {
            client,
        })
    }

    pub async fn close(self) -> ZKResult<()> {
        self.client.close().await?;
        Ok(())
    }

    pub async fn create(&self, path: &str, data: Option<&[u8]>, acl: Vec<ACL>, create_model: CreateMode) -> ZKResult<String> {
        Ok("/xjj".to_string())
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[tokio::test]
    async fn new_zk() {
        let mut zk = match ZooKeeper::new("127.0.0.1:2181", 5000).await {
            Ok(zk) => zk,
            Err(e) => {
                error!("error in new zk {:?}", e);
                return;
            },
        };
        zk.close().await;
        thread::sleep(Duration::from_secs(3));
        info!("{:?}", zk);
    }
}