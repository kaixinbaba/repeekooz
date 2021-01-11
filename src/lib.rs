//! **buruma** 是一个使用 Rust [ZooKeeper](https://zookeeper.apache.org/) 异步客户端，基本实现了 Java 版官方
//! 客户端的功能
//!
//! 引入依赖
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
//! ## 创建 ZK 客户端
//!
//! ```rust,no_run
//! extern crate buruma;
//! use buruma::ZooKeeper;
//! use buruma::ZKResult;
//!
//! #[tokio::main]
//! async fn main() -> ZKResult<()> {
//!     let mut zk = ZooKeeper::new("127.0.0.1:2181", 5000).await.unwrap();
//!     Ok(())     
//! }
//! ```

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// re-export, 常用的结构体全部都要导出
pub use api::ZooKeeper;
pub use constants::{EventType, KeeperState};
pub use watcher::{WatchedEvent, Watcher};

use crate::constants::Error;

pub mod api;
mod client;
pub mod constants;
mod metric;
mod paths;
pub mod protocol;
mod watcher;

#[derive(Debug)]
pub struct ZKError(Error, &'static str);

pub type ZKResult<T> = Result<T, ZKError>;
