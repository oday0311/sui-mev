### start_bot功能分析
1. 初始化设置 ：
   
   - 使用 utils::set_panic_hook() 设置panic钩子。
   - 使用 mev_logger::init_with_whitelisted_modules() 初始化日志记录器，指定了模块的白名单。
2. 密钥对处理 ：
   
   - 从传入的参数中解码私钥以生成 SuiKeyPair 。
   - 从密钥对中提取公钥，并生成攻击者地址 SuiAddress 。
3. 日志记录 ：
   
   - 使用 info! 宏记录启动信息，包括攻击者地址和各种配置参数。
4. 引擎初始化 ：
   
   - 创建一个默认的 Engine 实例。
   - 根据配置参数，添加不同的收集器和执行器到引擎中。
5. 模拟器池 ：
   
   - 根据配置参数决定使用 DBSimulator 或 HttpSimulator 来初始化模拟器池。
   - 使用 ObjectPool 管理模拟器实例。
6. 策略和执行器 ：
   
   - 创建 ArbStrategy 实例并添加到引擎中。
   - 添加 TelegramMessageDispatcher 执行器用于通知。
7. 心跳和运行 ：
   
   - 启动心跳功能以定期发送心跳信号。
   - 运行引擎并等待其完成。
### 总结
start_bot 功能主要负责初始化和配置一个交易套利引擎。它通过设置日志记录、处理密钥对、配置收集器和执行器、管理模拟器池以及启动心跳功能来实现这一点。整个过程涉及多个组件的协作，以实现自动化的交易处理和套利策略执行。

如果需要更详细的分析或具体代码实现，请提供相关代码片段或文件。








### ArbStrategy 结构体
ArbStrategy 是一个用于处理套利机会的策略结构体。它包含以下主要组件：

- sender : 攻击者的地址。
- arb_item_sender : 用于发送套利项的异步通道发送器。
- arb_cache : 用于缓存套利项的缓存。
- recent_arbs : 最近处理过的套利项的队列。
- simulator_pool : 模拟器池，用于模拟交易。
- own_simulator : 自己的模拟器实例，用于执行挂起的交易。
- rpc_url : RPC服务的URL。
- workers : 工作线程的数量。
- sui : Sui客户端实例。
- epoch : 当前的模拟纪元。
- dedicated_simulator : 专用的重放模拟器。
### ArbStrategy 的方法
1. new : 初始化一个新的 ArbStrategy 实例，设置各种参数和组件。
2. on_new_tx : 处理新的交易数据，模拟交易并解析结果。
3. on_new_tx_effects : 处理新的交易效果，解析涉及的币池并更新缓存。
4. on_new_shio_item : 处理新的Shio项，识别潜在的套利机会并更新缓存。
5. parse_involved_coin_pools : 解析事件中涉及的币池。
6. get_potential_opportunity : 获取潜在的套利机会。
7. get_latest_epoch : 获取最新的模拟纪元。
8. sync_state : 同步策略状态，初始化工作线程。
9. process_event : 处理事件，根据事件类型调用相应的方法。