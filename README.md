# buruma
ZooKeeper client written in pure Rust for humans

![](./buruma.png)

## Basic Usage
引入依赖，在 `Cargo.toml`
```
buruma = "0.1.0"
```
```rust
use buruma::{ACL, CreateMode, ZooKeeper};
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
