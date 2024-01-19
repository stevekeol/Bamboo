# Modules

> Introduce the core modules and interaction relationship in the architecture overview.


- Data: MarketGenerator trait管理MarketEvents的生成，该事件作为系统的心跳。例如，提供了live::MarketFeed实现，该实现利用[Barter-Data] WebSocket集成来提供实时交易数据（例如交易、蜡烛图等）。
- Strategy: SignalGenerator trait管理对传入MarketEvents进行分析后潜在信号的生成。信号是建议性的，并发送到Portfolio进行分析。
- Portfolio: MarketUpdater、OrderGenerator和FillUpdater管理全局状态Portfolio的实现。Portfolio在接收到来自Strategy的建议性SignalEvents后可能生成OrderEvents。Portfolio在接收MarketEvents和FillEvents后更新状态。
- Execution: ExecutionClient trait管理在接收到来自Portfolio的OrderEvents后生成FillEvents。例如，提供了SimulatedExecution处理程序的实现，用于模拟在干式交易或回测运行中所需的任何交易执行行为。
- Statistic: 提供指标，如夏普比率、卡尔马比率和最大回撤，以分析交易会话的性能。一次通过的离散算法分析每个已关闭的头寸，并高效地计算交易摘要。
- Trader: 能够使用自己的Data、Strategy和Execution实例以及对全局Portfolio的共享访问，交易单个市场对。
- Engine: 多线程交易引擎，能够与任意数量的Trader市场对进行交易。每个包含的Trader实例在自己的线程上运行。


- Engine应该限制为一个Trader【交易员】，除非你希望针对每个Market(即Venue)都有一个Trader。
- Engine和Trader(s)的配置是硬编码而不是加载配置的(当然需要TODO)
- 远程Commands（是一些枚举类型，比如Command::Terminate, Command::ExitPosition等）可以通过command_tx发送给Engine，来对其控制。