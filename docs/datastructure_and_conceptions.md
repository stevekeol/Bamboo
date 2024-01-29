# data-structure & 概念

> Bamboo项目中一些数据结构的意义和定位

> 注: Symbol，即数字货币的符号。也可以表示交易对，如BTCUSDT。

```rust title="Instrument: 金融资产(股票，债券，期权等)"
/// 比如对于BTCUSDT这个交易对的现货交易(Spot)
struct Instrument {
	base: String,  // 基础货币(BTC)
	quote: String, // 报价货币(USDT)
	kind: String,  // 金融工具类型(Spot-现货, stock-股票, Bond-债券, Option-期权, Future-期货, Forex-外汇 ...)
}
```

```rust title="InstrumentKind: 金融工具的类型(现货，期货，合约等)"
enum InstrumentKind {
    Spot,                   /*现货*/
    Future(FutureContract), /*期货合约*/
    Perpetual,              /*永续合约*/
    Option(OptionContract), /*期权合约*/
}
```

```rust title="Market: 市场 = 交易场所 + 交易资产 (两者共同约定)"
enum Exchange {
	Binance,
	Bitfinex,
	Bitmex,
	Bybit,
	Coinbase,
	Gateio,
	Kraken,
	Okx,
	...
}

struct Market {
	exchange: Exchange,     // 交易所
	instrument: Instrument, // 金融资产
}
```

```rust title="OrderKind(OrderType): 投资者根据交易策略和风险偏好，来选择下单的订单种类"
enum OrderKind {
	Market,  // 市价订单: 即以市场当前可用的最佳价格成交
	Limit,   // 限价订单: 指定买入卖出的价格(故可能无法立即成交)
	Bracket, // 挂接订单: 包含止损止盈条件的订单
}
```

```rust title="Portfolio: 投资组合的元数据"
struct Portfolio {
	engine_id: String,  //
	markets: []Market,  //
	starting_cash,      //
	repository,         //
	allocation_manager, //
	risk_manager,       //
	statictic_config,   //
}
```

## 概念

1. 通道
在币安提供的WebSocket API中，所谓的【通道】（Channel）指的是你可以订阅以接收特定类型数据的标识符或主题。通过订阅不同的通道，你可以获取不同类型的实时市场数据，比如交易更新、深度信息、K线数据等。每个通道都对应着特定类型的数据流，订阅相应的通道可以让你实时获取你所需的市场信息，以便进行实时监控和交易决策。

2. 交易更新
【交易更新】指的是市场上发生的交易事件的实时更新。这包括买单和卖单的成交情况，比如新的交易订单、部分成交、完全成交等。通过订阅交易更新的通道，你可以获取市场上最新的交易执行情况，以便及时了解市场动态。

3. 深度信息
【深度信息】（Order Book Depth）指的是市场上买单和卖单的挂单情况，包括每个价格点上的买单和卖单的数量和价格。深度信息可以帮助你了解市场上的供需情况，以及市场对特定价格的敏感度，是进行交易决策和风险管理的重要数据来源。

4. 订单簿
订单簿是指交易所或市场上的所有买单和卖单的集合。它记录了所有挂单的价格和数量，以及待成交的交易订单。订单簿通常分为买单和卖单两部分，买单部分记录了买家愿意以不同价格购买资产的挂单情况，而卖单部分记录了卖家愿意以不同价格出售资产的挂单情况。订单簿的深度信息可以帮助交易者了解市场上的供需情况，以及市场对特定价格的敏感度，是进行交易决策和风险管理的重要参考数据。


5. PublicTrades和OrderBooksL1
PublicTrades和OrderBooksL1是与交易所市场数据相关的术语。
- PublicTrades（公开交易）通常指代交易所上公开可见的实际交易记录，包括成交价格、成交数量和成交时间等信息。
- OrderBooksL1（订单簿级别1）是指订单簿中最基本的信息，通常包括最佳买卖价和相应的订单量等数据。

在交易所中，OrderBooksL1，OrderBooksL2和OrderBooksL3代表订单簿的不同级别：

- OrderBooksL1（订单簿级别1）通常包括最佳的买入和卖出价格，以及相应的订单量。
- OrderBooksL2（订单簿级别2）通常包括更深层次的订单簿信息，包括最佳价格以及接下来的一些买入和卖出价格和相应的订单量。
- OrderBooksL3（订单簿级别3）通常包括最详细的订单簿信息，可能会包括更多的深度和细节，例如特定价格点上每个订单的数量等。

【Bamboo】项目中: 
 - Level1指的是OrderBook上每一侧(买/卖)的最佳的非聚合的Bid和Ask.
 - L2指的是按价格聚合的OrderBook.
 - Level3指的是非聚合的OrderBook，是直接从交易所复制过来的.

 > 按价格聚合的订单簿是指在订单簿中将相同价格的订单合并在一起，以显示同一价格下的总订单量。这种聚合可以帮助交易者更清晰地了解市场深度和价格趋势，同时减少了订单簿中条目的数量，使其更易于处理和分析。