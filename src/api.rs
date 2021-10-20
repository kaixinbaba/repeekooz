//! # ZooKeeper API 模块
//! 作为整个项目的入口文件，提供友好的 API 方法用于操作 ZK。

use std::time::Duration;

use bytes::BytesMut;

use crate::{paths, WatchedEvent, WatcherType, ZKError, ZKResult};
use crate::client::Client;
use crate::constants::{AddWatchMode, CreateMode, IGNORE_VERSION, OpCode, States};
use crate::error::ServerErrorCode;
use crate::protocol::req::{
    ACL, AddWatchRequest, CheckWatchesRequest, CreateRequest, DeleteRequest,
    PathAndWatchRequest, PathRequest, RequestHeader, SetACLRequest, SetDataRequest,
};
use crate::protocol::resp::{
    CreateResponse, DummyResponse, GetACLResponse, GetAllChildrenNumberResponse,
    GetChildren2Response, GetDataResponse, IgnoreResponse, PathListResponse, SetDataResponse, Stat,
};
use crate::protocol::Serializer;
use crate::watcher::Watcher;

/// 整个模块的 API 入口对象
#[derive(Debug)]
pub struct ZooKeeper {
    client: Client,
}

#[derive(Debug, Hash)]
struct DummyWatcher;

impl Watcher for DummyWatcher {
    fn process(&self, _: &WatchedEvent) -> ZKResult<()> {
        Ok(())
    }
}

impl ZooKeeper {
    /// 创建 ZooKeeper 客户端
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(5)).await.unwrap();
    /// ```
    ///
    /// # Args
    /// - `connect_string`: 连接字符串格式为 "ip1:port1,ip2:port2,ip3:port3.../chroot"，其中 chroot 为可选
    /// - `session_timeout`: 会话超时时间, 参考 [`Duration`]     
    /// # Returns
    /// - `ZooKeeper`：ZooKeeper 客户端对象
    /// # Errors
    ///
    /// 无法连接服务端或者连接字符串格式有问题将会返回异常
    pub async fn new(connect_string: &str, session_timeout: Duration) -> ZKResult<ZooKeeper> {
        pretty_env_logger::try_init();
        let client = Client::new(connect_string, session_timeout.as_millis() as u32).await?;
        Ok(ZooKeeper { client })
    }

    /// 创建目标路径的节点，数据是可选的
    /// # Examples
    /// ```rust,ignore
    /// // 创建一个没有数据的节点
    /// let path = zk.create("/a/new/path", None, ACL::world_acl(), CreateMode::Persistent)
    ///        .await
    ///        .unwrap();
    /// // 创建一个带数据的节点
    /// let path = zk.create("/your/path", Some("buruma".as_bytes()), ACL::world_acl(), CreateMode::Persistent)
    ///        .await
    ///        .unwrap();
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `data`： 节点的数据，可选
    /// - `acl`： 该节点的权限数据，可以有多个，参考 [`ACL`]
    /// - `create_model`： 节点的模式，参考 [`CreateMode`]
    /// # Returns
    /// - `String`：目标路径，同参数 `path`
    pub async fn create(
        &mut self,
        path: &str,
        data: Option<&[u8]>,
        acl_list: Vec<ACL>,
        create_model: CreateMode,
    ) -> ZKResult<String> {
        paths::validate_path(path)?;
        let rtype = match create_model {
            CreateMode::Container => OpCode::CreateContainer,
            _ => OpCode::Create,
        };
        let rh = Some(RequestHeader::new(rtype));
        let mut req = BytesMut::new();
        let request =
            CreateRequest::new_full(self.client.get_path(path), data, acl_list, create_model);
        request.write(&mut req);
        let resp = CreateResponse::default();
        let resp = self.client.submit_request(rh, req, resp).await?;
        Ok(resp.path)
    }

    /// 删除目标路径的节点数据
    /// # Examples
    /// ```rust,ignore
    /// zk.delete("/your/path").await;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    pub async fn delete(&mut self, path: &str) -> ZKResult<()> {
        self.deletev(path, IGNORE_VERSION).await?;
        Ok(())
    }

    /// 删除目标路径的节点数据携带版本条件，满足版本号才能删除
    /// # Examples
    /// ```rust,ignore
    /// zk.deletev("/your/path", 1024).await;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `version`： 节点指定的版本号，-1 为忽略版本
    pub async fn deletev(&mut self, path: &str, version: i32) -> ZKResult<()> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(OpCode::Delete));
        let mut req = BytesMut::new();
        let request = DeleteRequest::new(self.client.get_path(path), version);
        request.write(&mut req);
        let resp = IgnoreResponse::default();
        self.client.submit_request(rh, req, resp).await?;
        Ok(())
    }

    /// 为目标路径设置数据
    /// # Examples
    /// ```rust,ignore
    /// let stat = zk.set("/your/path", "Chinese Stand Up".as_bytes()).await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `data`： 节点数据
    /// # Returns
    /// - `Stat`： 统计对象，请查看 [`Stat`]
    pub async fn set(&mut self, path: &str, data: &[u8]) -> ZKResult<Stat> {
        self.setv(path, data, IGNORE_VERSION).await
    }

    /// 为目标路径设置数据，携带版本条件，满足版本号才能设置成功
    /// # Examples
    /// ```rust,ignore
    /// let stat = zk.setv("/your/path", "Chinese Stand Up".as_bytes(), 19491001).await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `data`： 节点数据
    /// - `version`： 节点指定的版本号，-1 为忽略版本
    /// # Returns
    /// - `Stat`： 统计对象，请查看 [`Stat`]
    pub async fn setv(&mut self, path: &str, data: &[u8], version: i32) -> ZKResult<Stat> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(OpCode::SetData));
        let mut req = BytesMut::new();
        let request = SetDataRequest::new(self.client.get_path(path), data, version);
        request.write(&mut req);
        let resp = SetDataResponse::default();
        let resp = self.client.submit_request(rh, req, resp).await?;
        Ok(resp.stat)
    }

    /// 获取目标路径数据，不需要回调
    /// # Examples
    /// ```rust,ignore
    /// let data = zk.get("/your/path", None).await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `stat`： 统计数据，可选，如果不为 None 则会将节点统计结果写入该对象, 关于更多统计对象，请查看 [`Stat`]
    /// # Returns
    /// - `Vec<u8>`： 目标节点的数据以字节数组的形式
    pub async fn get(&mut self, path: &str, stat: Option<&mut Stat>) -> ZKResult<Vec<u8>> {
        self.getw(path, None::<DummyWatcher>, stat).await
    }

    /// 获取目标路径数据，需要回调通知
    /// # Examples
    /// ```rust,ignore
    /// let data = zk.getw("/your/path", Some(YourWatcherImpl), None).await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `watcher`： 回调对象，必须实现 [`Watcher`] trait，可选
    /// - `stat`： 统计数据，可选，如果不为 None 则会将结果写入该对象, 关于更多统计对象，请查看 [`Stat`]
    /// # Returns
    /// - `Vec<u8>`： 目标节点的数据以字节数组的形式
    pub async fn getw(
        &mut self,
        path: &str,
        watcher: Option<impl Watcher + 'static>,
        stat: Option<&mut Stat>,
    ) -> ZKResult<Vec<u8>> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(OpCode::GetData));
        let mut req = BytesMut::new();
        let full_path = self.client.get_path(path);
        let watch = match watcher {
            Some(w) => {
                // 注册本地回调
                self.client
                    .register_data_watcher(full_path.clone(), Box::new(w))?;
                true
            }
            _ => false,
        };
        let request = PathAndWatchRequest::new(full_path, watch);
        request.write(&mut req);
        let resp = GetDataResponse::default();
        let resp = self.client.submit_request(rh, req, resp).await?;
        if let Some(s) = stat {
            *s = resp.stat;
        }
        Ok(resp.data)
    }

    /// 判断目标路径是否存在，不需要回调
    /// # Examples
    /// ```rust,ignore
    /// let stat = zk.exists("/your/path").await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// # Returns
    /// - `Stat`： 统计对象，请查看 [`Stat`]
    pub async fn exists(&mut self, path: &str) -> ZKResult<Option<Stat>> {
        self.existsw(path, None::<DummyWatcher>).await
    }

    /// 判断目标路径是否存在，需要回调通知
    /// # Examples
    /// ```rust,ignore
    /// let stat = zk.existsw("/your/path", Some(YourWatcherImpl), None).await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `watcher`： 回调对象，必须实现 [`Watcher`] trait，可选
    /// # Returns
    /// - `Stat`： 统计对象，请查看 [`Stat`]
    pub async fn existsw(
        &mut self,
        path: &str,
        watcher: Option<impl Watcher + 'static>,
    ) -> ZKResult<Option<Stat>> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(OpCode::Exists));
        let mut req = BytesMut::new();
        let full_path = self.client.get_path(path);
        let watch = match watcher {
            Some(w) => {
                // 注册本地回调
                self.client
                    .register_exists_watcher(full_path.clone(), Box::new(w))?;
                true
            }
            _ => false,
        };
        let request = PathAndWatchRequest::new(full_path, watch);
        request.write(&mut req);
        let resp = SetDataResponse::default();
        match self.client.submit_request(rh, req, resp).await {
            Ok(resp) => Ok(Some(resp.stat)),
            Err(e) => match e {
                ZKError::ServerError(ServerErrorCode::NoNode, _) => Ok(None),
                _ => {
                    return Err(e);
                }
            },
        }
    }

    /// 获取节点的子节点列表，不需要回调
    /// # Examples
    /// ```rust,ignore
    /// let children_list = zk.children("/your/path").await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// # Returns
    /// - `Vec<String>`： 子节点列表
    pub async fn children(&mut self, path: &str) -> ZKResult<Vec<String>> {
        self.childrenw(path, None::<DummyWatcher>).await
    }

    /// 获取节点的子节点列表，需要回调通知
    /// # Examples
    /// ```rust,ignore
    /// let stat = zk.childrenw("/your/path", Some(YourWatcherImpl), None).await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `watcher`： 回调对象，必须实现 [`Watcher`] trait，可选
    /// # Returns
    /// - `Vec<String>`： 子节点列表
    pub async fn childrenw(
        &mut self,
        path: &str,
        watcher: Option<impl Watcher + 'static>,
    ) -> ZKResult<Vec<String>> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(OpCode::GetChildren));
        let mut req = BytesMut::new();
        let full_path = self.client.get_path(path);
        let watch = match watcher {
            Some(w) => {
                // 注册本地回调
                self.client
                    .register_child_watcher(full_path.clone(), Box::new(w))?;
                true
            }
            _ => false,
        };
        let request = PathAndWatchRequest::new(full_path, watch);
        request.write(&mut req);
        let resp = PathListResponse::default();
        let resp = self.client.submit_request(rh, req, resp).await?;
        Ok(resp.path_list)
    }

    /// 获取节点的子节点列表，不需要回调，可以写入 stat
    /// # Examples
    /// ```rust,ignore
    /// let stat = Stat::default();
    /// let children_list = zk.childrens("/your/path", stat).await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `stat`： 统计数据，统计结果会写入该对象, 关于更多统计对象，请查看 [`Stat`]
    /// # Returns
    /// - `Vec<String>`： 子节点列表
    pub async fn childrens(&mut self, path: &str, stat: &mut Stat) -> ZKResult<Vec<String>> {
        self.childrensw(path, None::<DummyWatcher>, stat).await
    }

    /// 获取节点的子节点列表，需要回调通知，可以写入 stat
    /// # Examples
    /// ```rust,ignore
    /// let stat = Stat::default();
    /// let children_list = zk.childrensw("/your/path", Some(YourWatcherImpl), stat).await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `watcher`： 回调对象，必须实现 [`Watcher`] trait，可选
    /// - `stat`： 统计数据，统计结果会写入该对象, 关于更多统计对象，请查看 [`Stat`]
    /// # Returns
    /// - `Vec<String>`： 子节点列表
    pub async fn childrensw(
        &mut self,
        path: &str,
        watcher: Option<impl Watcher + 'static>,
        stat: &mut Stat,
    ) -> ZKResult<Vec<String>> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(OpCode::GetChildren2));
        let mut req = BytesMut::new();
        let full_path = self.client.get_path(path);
        let watch = match watcher {
            Some(w) => {
                // 注册本地回调
                self.client
                    .register_child_watcher(full_path.clone(), Box::new(w))?;
                true
            }
            _ => false,
        };
        let request = PathAndWatchRequest::new(full_path, watch);
        request.write(&mut req);
        let resp = GetChildren2Response::default();
        let resp = self.client.submit_request(rh, req, resp).await?;
        *stat = resp.stat;
        Ok(resp.path_list)
    }

    /// 获取目标路径下的所有子节点数量（包括孙子节点）
    /// # Examples
    /// ```rust,ignore
    /// let total_number = zk.children_count("/your/path").await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// # Returns
    /// - `u32`： 目标路径下的所有子节点数量
    pub async fn children_count(&mut self, path: &str) -> ZKResult<u32> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(OpCode::GetAllChildrenNumber));
        let mut req = BytesMut::new();
        let full_path = self.client.get_path(path);
        let request = PathRequest::new(full_path);
        request.write(&mut req);
        let resp = GetAllChildrenNumberResponse::default();
        let resp = self.client.submit_request(rh, req, resp).await?;
        Ok(resp.total_number)
    }

    /// 获取目标路径前缀下的所有临时节点（包括孙子节点）
    /// # Examples
    /// ```rust,ignore
    /// let path_list = zk.get_ephemerals("/your/path").await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头，不会拼接 chroot
    /// # Returns
    /// - `Vec<String>`： 所有符合条件临时节点的列表
    pub async fn get_ephemerals(&mut self, path: &str) -> ZKResult<Vec<String>> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(OpCode::GetEphemerals));
        let mut req = BytesMut::new();
        // 不需要拼接 chroot
        let request = PathRequest::new(path.to_string());
        request.write(&mut req);
        let resp = PathListResponse::default();
        let resp = self.client.submit_request(rh, req, resp).await?;
        Ok(resp.path_list)
    }

    /// 获取目标路径的权限信息
    /// # Examples
    /// ```rust,ignore
    /// let acl_list = zk.get_acl("/your/path", None).await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `stat`： 统计数据，可选，如果不为 None 则会将结果写入该对象, 关于更多统计对象，请查看 [`Stat`]
    /// # Returns
    /// - `Vec<ACL>`： 节点的 ACL 列表
    pub async fn get_acl(&mut self, path: &str, stat: Option<&mut Stat>) -> ZKResult<Vec<ACL>> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(OpCode::GetACL));
        let mut req = BytesMut::new();
        let full_path = self.client.get_path(path);
        let request = PathRequest::new(full_path);
        request.write(&mut req);
        let resp = GetACLResponse::default();
        let resp = self.client.submit_request(rh, req, resp).await?;
        if let Some(s) = stat {
            *s = resp.stat;
        }
        Ok(resp.acl_list)
    }

    /// 设置目标路径的权限信息
    /// # Examples
    /// ```rust,ignore
    /// let acl_list = zk.get_acl("/your/path", None).await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `acl_list`： 该节点的权限数据，可以有多个，参考 [`ACL`]
    /// # Returns
    /// - `Stat`： 统计对象，请查看 [`Stat`]
    pub async fn set_acl(
        &mut self,
        path: &str,
        acl_list: Vec<ACL>,
        version: i32,
    ) -> ZKResult<Stat> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(OpCode::SetACL));
        let mut req = BytesMut::new();
        let full_path = self.client.get_path(path);
        let request = SetACLRequest::new(full_path, acl_list, version);
        request.write(&mut req);
        let resp = SetDataResponse::default();
        let resp = self.client.submit_request(rh, req, resp).await?;
        Ok(resp.stat)
    }

    /// 为目标路径添加回调通知
    /// # Examples
    /// ```rust,ignore
    /// zk.add_watch("/your/path", YourWatcherImpl, AddWatchMode::Persistent).await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `watcher`： 回调对象，必须实现 [`Watcher`] trait
    /// - `mode`： 添加回调的种类，请查看 [`AddWatchMode`]
    pub async fn add_watch<W: Watcher + 'static>(
        &mut self,
        path: &str,
        watcher: W,
        mode: AddWatchMode,
    ) -> ZKResult<()> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(OpCode::AddWatch));
        let mut req = BytesMut::new();
        let full_path = self.client.get_path(path);
        self.client.register_persistent_watcher(
            full_path.clone(),
            Box::new(watcher),
            mode == AddWatchMode::PersistentRecursive,
        )?;
        let request = AddWatchRequest::new(full_path, mode);
        request.write(&mut req);
        self.client
            .submit_request(rh, req, DummyResponse::default())
            .await?;
        Ok(())
    }

    /// 删除目标路径上指定类型的回调通知
    /// # Examples
    /// ```rust,ignore
    /// zk.remove_watches("/your/path", YourWatcherImpl, WatcherType::Any, true).await?;
    /// ```
    ///
    /// # Args
    /// - `path`： 目标路径，必须以 "/" 开头
    /// - `watcher`： 回调对象，必须实现 [`Watcher`] trait
    /// - `watcher_type`： 回调的类型，请查看 [`WatcherType`]
    /// - `local`：
    pub async fn remove_watches<W: Watcher + 'static>(
        &mut self,
        path: &str,
        watcher: W,
        watcher_type: WatcherType,
        local: bool,
    ) -> ZKResult<()> {
        paths::validate_path(path)?;
        let rh = Some(RequestHeader::new(OpCode::CheckWatches));
        let mut req = BytesMut::new();
        let full_path = self.client.get_path(path);
        // Java 中的 watchDeregistration 怎么实现
        let request = CheckWatchesRequest::new(full_path, watcher_type);

        request.write(&mut req);
        self.client
            .submit_request(rh, req, DummyResponse::default())
            .await?;
        Ok(())
    }

    /// 获取客户端当前状态
    /// # Examples
    /// ```rust,ignore
    /// let state = zk.state()?;
    /// ```
    ///
    /// # Returns
    /// - `States`： 关于更多客户端状态，请查看 [`States`]
    pub fn state(&self) -> ZKResult<States> {
        Ok(self.client.state.clone())
    }

    /// 获取客户端当前 session_id
    /// # Examples
    /// ```rust,ignore
    /// let session_id = zk.session_id()?;
    /// ```
    ///
    /// # Returns
    /// - `i64`： 服务端分配的唯一会话 ID
    pub fn session_id(&self) -> ZKResult<i64> {
        Ok(self.client.session_id)
    }

    /// 获取客户端当前会话超时时间
    /// # Examples
    /// ```rust,ignore
    /// let session_timeout = zk.session_timeout()?;
    /// ```
    ///
    /// # Returns
    /// - `u32`： 客户端的会话超时时间
    pub fn session_timeout(&self) -> ZKResult<u32> {
        Ok(self.client.session_timeout)
    }
}
