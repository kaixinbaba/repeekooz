#![allow(clippy::upper_case_acronyms, clippy::type_complexity)]
//! **repeekooz** 是一个使用纯 Rust 编写的 [ZooKeeper](https://zookeeper.apache.org/) 异步客户端，基本实现了 Java 版官方
//! 客户端的功能
//!
//! # 引入依赖
//!
//! ```ini
//! [dependencies.repeekooz]
//! version = "*"
//! ```
//!
//!
//! # 基本操作
//!
//! repeekooz 支持 low-level 的基本接口以及更高级的 high-level 的高级接口和一些在分布式环境下的工具在 recipe 模块下
//!
//! ## 简单示例
//!
//! ```rust,ignore
//! extern crate repeekooz;
//! use repeekooz::ZooKeeper;
//! use repeekooz::ZKResult;
//! use std::time::Duration;
//! use repeekooz::CreateMode;
//! use repeekooz::ACL;
//!
//! #[tokio::main]
//! async fn main() -> ZKResult<()> {
//!     let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(5)).await.unwrap();
//!     let basic_path = "/repeekooz";
//!     // 创建节点
//!     let path = zk
//!        .create(basic_path, Some("repeekooz".as_bytes()), ACL::world_acl(), CreateMode::Persistent)
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

extern crate lazy_static;
#[macro_use]
extern crate log;

// re-export, 常用的结构体全部都要导出，使得用户可以直接通过 repeekooz 直接引用
pub use api::ZooKeeper;
pub use constants::{AddWatchMode, CreateMode, EventType, KeeperState, States, WatcherType};
pub use error::ZKError;
pub use protocol::req::{Scheme, ACL};
pub use protocol::resp::Stat;
pub use watcher::{WatchedEvent, Watcher};

use anyhow::Result;

mod api;
mod client;
mod constants;
mod error;
mod metric;
mod paths;
mod protocol;
mod recipes;
mod watcher;

pub type ZKResult<T> = Result<T, ZKError>;
