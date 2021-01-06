//! # ZooKeeper API 模块
//! 作为整个项目的入口文件，提供友好的 API 方法用于操作 ZK。
//! ## Example

use bytes::BytesMut;

use crate::client::Client;
use crate::constants::{CreateMode, Error, OpCode, IGNORE_VERSION};
use crate::protocol::req::{CreateRequest, DeleteRequest, RequestHeader, SetDataRequest, ACL};
use crate::protocol::resp::{CreateResponse, IgnoreResponse, SetDataResponse, Stat};
use crate::protocol::Serializer;
use crate::{paths, ZKError, ZKResult};

#[derive(Debug)]
pub struct ZooKeeper {
    client: Client,
}

impl ZooKeeper {
    /// 创建 ZooKeeper 客户端
    /// # Args
    /// - `connect_string`: 连接字符串格式为 "ip1:port1,ip2:port2,ip3:port3.../chroot"，其中 chroot 为可选
    /// - `session_timeout`: 会话超时时间     
    /// # Errors
    ///
    /// 无法连接服务端或者连接字符串格式有问题
    pub async fn new(connect_string: &str, session_timeout: i32) -> ZKResult<ZooKeeper> {
        pretty_env_logger::try_init();
        let client = Client::new(connect_string, session_timeout).await?;
        Ok(ZooKeeper { client })
    }

    /// 创建目标路径的节点，数据是可选的
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

    /// 删除目标路径的节点数据
    pub async fn delete(&mut self, path: &str) -> ZKResult<()> {
        self.delete_with_version(path, IGNORE_VERSION).await
    }

    /// 删除目标路径的节点数据携带版本条件，满足版本号才能删除
    pub async fn delete_with_version(&mut self, path: &str, version: i32) -> ZKResult<()> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(0, OpCode::Delete as i32));
        let mut req = BytesMut::new();
        let request = DeleteRequest::new(self.client.get_path(path), version);
        request.write(&mut req);
        let resp = IgnoreResponse::default();
        let (reply_header, _) = self.client.submit_request(rh, req, resp).await?;
        if reply_header.err != 0 {
            return Err(ZKError(
                Error::from(reply_header.err as isize),
                "Error from server",
            ));
        }
        Ok(())
    }

    /// 为目标路径设置数据
    pub async fn set_data(&mut self, path: &str, data: &[u8]) -> ZKResult<Stat> {
        self.set_data_with_version(path, data, IGNORE_VERSION).await
    }

    /// 为目标路径设置数据，携带版本条件，满足版本号才能设置成功
    pub async fn set_data_with_version(
        &mut self,
        path: &str,
        data: &[u8],
        version: i32,
    ) -> ZKResult<Stat> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(0, OpCode::SetData as i32));
        let mut req = BytesMut::new();
        let request = SetDataRequest::new(self.client.get_path(path), data, version);
        request.write(&mut req);
        let resp = SetDataResponse::default();
        let (reply_header, resp) = self.client.submit_request(rh, req, resp).await?;
        if reply_header.err != 0 {
            return Err(ZKError(
                Error::from(reply_header.err as isize),
                "Error from server",
            ));
        }
        Ok(resp.stat)
    }
}
