# TODO list
- 如何转成同步
- 补齐文档
- callback 函数 以及 对应的 event 处理流程
- watcher 的回调通知需要另一个请求去触发才能接收到？是不是 tokio 的 api 用的有问题 还是异步编程就是这样？
- PING 怎么解决 执行单元测试会直接卡死 应该和 tokio send 有关系