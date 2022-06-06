# buruma
使用 [Rust](https://www.rust-lang.org/) 基于 [tokio](https://tokio.rs/) 编写的 [ZooKeeper](https://zookeeper.apache.org/) 高性能异步客户端。

![女神](./buruma.png)

## Basic Usage
引入依赖，在 `Cargo.toml`
```
repeekooz = "0.1.0"
```
```rust
use repeekooz::{ACL, CreateMode, ZooKeeper};
// create client
let mut zk = ZooKeeper::new("127.0.0.1:2181", Duration::from_secs(5)).await?;
// create node
zk.create("/your/path", Some("I love buruma".as_bytes()), ACL::world_acl(), CreateMode::Persistent).await?;
// get data
let data = zk.get("/your/path", None).await?;
// set data
let stat = zk.set("/your/path", "buruma NB".as_bytes()).await?;
// delete node
zk.delete("/your/path").await?;
```
