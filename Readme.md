1. Event Stream 事件流概念以及应用

2. Kafka 提供的功能
- 发布订阅
- 永久性存储
- 可以即时或者回顾处理事件

3. Kafka 工作原理
- 分布式系统，保证高可用、防止单点故障带来的数据丢失
- 基于TCP协议


4. 主要概念和术语
**Event**
- Key
- Value
- Timestamp
- metadata(optional)

**Producers**
写入Event的客户端

**Consumers**
处理Event的客户端

**Topics**
- Topic是组织Event的逻辑容器
- 一个Topic通常有多个Producer和多个Consumer
- 与其他的消息队列不同，消息在消费后不会被删除，而是可以进行配置（topic级别）
- Kafka的性能不会受到数据大小的影响，因此可以长时间保留数据

**Partitions**


5. APIs
- AdminAPI
- ProducerAPI: 事件发布写入
- ConsumerAPI:  事件丁略处理
- Streams API: 
- Connect API: 数据导入和导出的连接器。例子： MySQL Source Connector 可以通过监听MySQL的表变化，将变化的数据写入Kafka

6. Push 和 Pull 两种方式
Kafka 选择让 Consumer 使用 Pull的方式从 Brokers 拉取数据
Push和Pull两种方式各有优缺点，在Push的模式下，当存在不同类型的 Consumer 时，很难处理Broker处理推送的速率。
我们的目标是让 Consumer 以尽可能快的速度处理数据，Push的方式可能会超过 Consumer 的最大处理速度。（这可以通过backoff的通信协议缓解，但要复杂的多）
Pull的方式可以让 Consumer 以自己的速度处理数据。

Pull系统的另一个好处是，Broker可以发送 多个 消息给 Consumer。而基于Push的系统，则需要选择发送 单个 或者 多个给 Downstream。



## Oula
1. 每一个Epoch创建对应的Topic, pool-solution-epoch-{number}, server根据当前所在的epoch，发送到相应的topic中。
2. Topic采用自动创建的形式，暂定为3个分区，1个副本。
3. 发送消息时只指定topic的名称，不指定分区，由kafka自动选择分区。

