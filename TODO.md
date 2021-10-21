# TODO list
- 如何转成同步
- set_data 这种要支持直接传入一个具体类型，然后通过一些可配置的序列化规则（例如：json）存入 data 字段
- get_data 这种要支持直接返回一个具体类型，然后通过一些可配置的序列化规则（例如：json）反序列化成对象结果
- 创建和删除 API 要支持递归
- addWatch 使用引用当场触发解决了只触发一次的情况，有没有更好的写法？
- 提供命令行工具解析 ZK 的快照文件和日志文件，并可以修改
- 参数错误是否要细分到每一个场景（例如：ip、host、port 都单独是一个枚举等等）
- zk path 要不仅仅是 &str，还可以是个 Path 对象类似（使用 Builder 模式创建）或者其他

# java client api
- [x] create
- [x] delete
- [x] getData
- [x] setData
- [x] exists
- [x] getChildren
- [x] getAllChildrenNumber
- [x] getEphemerals
- [x] getChildren2
- [x] getState
- [x] getSessionId
- [x] getSessionTimeout
- [x] getACL
- [x] setACL
- [x] addWatch
- [ ] removeWatches
- [ ] removeAllWatches
- [ ] getConfig
- [ ] updateServerList
- [ ] multi
- [ ] transaction
- [ ] sync

# java client async api
- [ ] create
- [ ] delete
- [ ] getData
- [ ] setData
- [ ] exists
- [ ] updateServerList
- [ ] getSessionId
- [ ] getSessionTimeout
- [ ] multi
- [ ] transaction
- [ ] getConfig
- [ ] getACL
- [ ] getChildren
- [ ] getAllChildrenNumber
- [ ] getEphemerals
- [ ] sync
- [ ] removeWatches
- [ ] removeAllWatches
- [ ] addWatch
- [ ] getState

