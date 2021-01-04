use bytes::BytesMut;

use crate::client::Client;
use crate::constants::{CreateMode, Error, OpCode, IGNORE_VERSION};
use crate::protocol::req::{CreateRequest, RequestHeader, ACL};
use crate::protocol::resp::CreateResponse;
use crate::protocol::Serializer;
use crate::{paths, ZKError, ZKResult};

#[derive(Debug)]
pub struct ZooKeeper {
    client: Client,
}

impl ZooKeeper {
    pub async fn new(connect_string: &str, session_timeout: i32) -> ZKResult<ZooKeeper> {
        pretty_env_logger::try_init();
        let client = Client::new(connect_string, session_timeout).await?;
        Ok(ZooKeeper { client })
    }

    pub async fn create(
        &mut self,
        path: &str,
        data: Option<&[u8]>,
        acl: Vec<ACL>,
        create_model: CreateMode,
    ) -> ZKResult<String> {
        paths::validate_path(path)?;
        let rtype = match create_model {
            CreateMode::Container => OpCode::CreateContainer,
            _ => OpCode::Create,
        };
        let rh = Some(RequestHeader::new(0, rtype as i32));
        let mut req = BytesMut::new();
        let request = CreateRequest::new_full(self.client.get_path(path), data, acl, create_model);
        request.write(&mut req);
        let resp = CreateResponse::default();
        let (reply_header, resp) = self.client.submit_request(rh, req, resp).await?;
        if reply_header.err != 0 {
            return Err(ZKError(
                Error::from(reply_header.err as isize),
                "Error from server",
            ));
        }
        Ok(resp.path)
    }

    pub async fn delete(&mut self, path: &str) -> ZKResult<()> {
        self.delete_with_version(path, IGNORE_VERSION).await
    }

    pub async fn delete_with_version(&mut self, path: &str, version: i32) -> ZKResult<()> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(0, OpCode::Delete as i32));
        Ok(())
    }
}
