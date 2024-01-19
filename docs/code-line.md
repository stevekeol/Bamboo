# Code line

```yaml title="代码量统计"
├── lib.rs: 100
├── event.rs: 100
├── data(180)
│ ├── error.rs: 40
│ ├── historical.rs: 40
│ ├── live.rs: 50
│ └── mod.rs: 50
├── engine(900)
│ ├── error.rs: 
│ ├── mod.rs: 500
│ └── trader.rs: 400
├── execution(320)
│ ├── error.rs: 
│ ├── mod.rs: 170
│ └── simulated.rs: 150
├── portfolio(1600) 
│ ├── allocator.rs: 200
│ ├── error.rs: 
│ ├── mod.rs: 230
│ ├── portfolio.rs: 550 # 注释 1000
│ ├── position.rs: 600 # 注释 1000
│ ├── repository(500)
│ │ ├── error.rs: 
│ │ ├── in_memory.rs: 140
│ │ ├── mod.rs: 80
│ │ └── redis.rs: 250
│ └── risk.rs: 40
├── statistic(120)
│ ├── algorithm.rs: 40 # 注释 200
│ ├── dispersion.rs: 80 # 注释 150
│ ├── error.rs: 
│ ├── metric(400)
│ │ ├── drawdown.rs: 190 # 注释 400
│ │ ├── mod.rs: 50 # 注释 100
│ │ └── ratio.rs: 140 # 注释 300
│ ├─ mod.rs: 30
│ └── summary(530)
│     ├── data.rs: 50 # 注释 100
│     ├── drawdown.rs: 60
│     ├── mod.rs: 80
│     ├── pnl.rs: 200 # 注释 50
│     └── trading.rs: 140
└── strategy(190)
    ├── example.rs: 90
    └── mod.rs: 100 # 注释 50
```