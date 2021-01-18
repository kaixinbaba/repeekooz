//! **buruma** 是一个使用纯 Rust 编写的 [ZooKeeper](https://zookeeper.apache.org/) 异步客户端，基本实现了 Java 版官方
//! 客户端的功能
//!
//! # 引入依赖
//!
//! ```ini
//! [dependencies.buruma]
//! version = "*"
//! ```
//!
//!
//! # 基本操作
//!
//! buruma 支持 low-level 的基本接口以及更高级的 high-level 的高级接口和一些在分布式环境下的工具在 recipe 模块下
//!
//! ## 简单示例
//!
//! ```rust
//! extern crate buruma;
//! use buruma::ZooKeeper;
//! use buruma::ZKResult;
//! use std::time::Duration;
//! use buruma::CreateMode;
//! use buruma::ACL;
//!
//! #[tokio::main]
//! async fn main() -> ZKResult<()> {
//!     let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(5)).await.unwrap();
//!     let basic_path = "/buruma";
//!     // 创建节点
//!     let path = zk
//!        .create(basic_path, Some("buruma".as_bytes()), ACL::world_acl(), CreateMode::Persistent)
//!        .await
//!        .unwrap();
//!     // 查询节点
//!     let result = zk.get(basic_path, None).await.unwrap();
//!     // 设置节点数据
//!     let stat = zk.set(basic_path, "kaixinbaba".as_bytes()).await.unwrap();
//!     // 删除节点
//!     zk.delete(basic_path);
//!     Ok(())     
//! }
//! ```

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// re-export, 常用的结构体全部都要导出，使得用户可以直接通过 buruma 直接引用
pub use api::ZooKeeper;
pub use constants::{CreateMode, EventType, KeeperState, States};
pub use protocol::req::{Scheme, ACL};
pub use protocol::resp::Stat;
pub use watcher::{WatchedEvent, Watcher};

use crate::constants::Error;

mod api;
mod client;
mod constants;
mod metric;
mod paths;
mod protocol;
mod watcher;

#[derive(Debug)]
pub struct ZKError(Error, &'static str);

pub type ZKResult<T> = Result<T, ZKError>;
