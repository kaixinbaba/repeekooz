# TODO list
- 如何转成同步
- 补齐文档
- api 的函数名要尽量简写
- callback 函数 以及 对应的 event 处理流程
- set_data 这种要支持直接传入一个具体类型，然后通过一些可配置的序列化规则（例如：json）存入 data 字段
- get_data 这种要支持直接返回一个具体类型，然后通过一些可配置的序列化规则（例如：json）反序列化成对象结果


# client api
- [x] create
- [x] delete
- [x] getData
- [x] setData
- [x] exists
- [ ] updateServerList
- [ ] getSessionId
- [ ] getSessionTimeout
- [ ] multi
- [ ] transaction
- [ ] getConfig
- [ ] getACL
- [ ] getChildren
- [ ] getChildren2
- [ ] getAllChildrenNumber
- [ ] getEphemerals
- [ ] sync
- [ ] removeWatches
- [ ] removeAllWatches
- [ ] addWatch
- [ ] getState

# client async api
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