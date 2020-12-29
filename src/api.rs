use crate::client::Client;
use crate::ZKResult;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use tokio::time::Duration;
use crate::protocol::req::{ACL, CreateRequest, RequestHeader};
use crate::constants::{CreateMode, Error, OpCode};
use crate::protocol::resp::CreateResponse;
use bytes::BytesMut;
use crate::protocol::Serializer;

#[derive(Debug)]
pub struct ZooKeeper {
    client: Client,
}

impl ZooKeeper {
    pub async fn new(connect_string: &str, session_timeout: i32) -> ZKResult<ZooKeeper> {
        pretty_env_logger::try_init();
        let client = Client::new(connect_string, session_timeout).await?;
        Ok(ZooKeeper {
            client,
        })
    }

    pub async fn create(&mut self, path: &str, data: Option<&[u8]>, acl: Vec<ACL>, create_model: CreateMode) -> ZKResult<String> {

        let rtype = match create_model {
            CreateMode::Container => OpCode::CreateContainer,
            _ => OpCode::Create,
        };
        let rh = Some(RequestHeader::new(0, rtype as i32));
        let mut req= BytesMut::new();
        let mut request = CreateRequest::new_full(path, data, acl, create_model);
        request.write(&mut req);
        let resp = CreateResponse::default();
        let (reply_header, resp) = self.client.submit_request(rh, req, resp).await?;
        if reply_header.err != 0 {
            return Err(Error::from(reply_header.err as isize));
        }
        Ok(resp.path)
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
        thread::sleep(Duration::from_secs(10));
        info!("{:?}", zk);
    }

    #[tokio::test]
    async fn create() {
        let mut zk = ZooKeeper::new("127.0.0.1:2181", 60000).await.unwrap();
        info!("{:?}", zk);
        let data = Some("I Love U".as_bytes());
        let path = zk.create("/xjj", data, ACL::world_acl(), CreateMode::EphemeralSequential).await.unwrap();
        info!("path: {}", path);
        thread::sleep(Duration::from_secs(10));
    }
}