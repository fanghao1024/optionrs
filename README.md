# optionrs

A high-performance, production-grade option pricing library for Rust, supporting classic pricing models (Black-Scholes, Binomial Tree, Monte Carlo, PDE numerical calculation) and exotic options (barrier, Asian, forward-start,etc).

[![Crates.io](https://img.shields.io/crates/v/optionrs.svg)](https://crates.io/crates/optionrs)
[![Docs.rs](https://docs.rs/optionrs/badge.svg)](https://docs.rs/optionrs)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](./LICENSE)
[![CI](https://github.com/fanghao1024/optionrs/actions/workflows/ci.yml/badge.svg)](https://github.com/fanghao1024/optionrs/actions)

## Features
- **Core Pricing Models**:
    - Black-Scholes (European calls/puts, Greeks, implied volatility)
    - Binomial Tree (European/American options, Delta/Gamma)
    - Monte Carlo Simulation (European/exotic options, path-dependent pricing)
- **Exotic Options**: Barrier, Asian (discrete geometric average), forward-start options
- **Numerical Stability**: Robust boundary condition handling (T=0, sigma=0)
- **Type Safety**: Clear, documented APIs with financial semantics
- **Test Coverage**: Full unit/integration/doc tests, validated against classic financial benchmarks

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
assert_approx_eq = "1.1.0"
owens-t = "0.1.5"
rand = "0.9.2"
rand_distr = "0.5.1"
rayon = "1.11.0"
statrs = "0.18.0"
```
## Module Catalog
```Plain Text
src/
├── lib.rs                 # 库入口，导出公共API
├── errors.rs              # 错误处理机制
├── products/              # 产品层：定义具体期权产品
│   ├── mod.rs
│   ├── european.rs        # 欧式期权
│   ├── american.rs        # 美式期权
│   ├── barrier.rs         # 障碍期权
│   ├── lookback.rs        # 回望期权
│   ├── spread.rs          # 价差期权
│   └── exotic.rs          # 其他奇异期权
├── core/                  # 引擎层：定价引擎实现
│   ├── mod.rs
│   ├── engine.rs          # 定价引擎接口
│   ├── analytic.rs        # 解析解引擎
│   ├── binomial.rs        # 二叉树引擎
│   ├── monte_carlo.rs     # 蒙特卡洛引擎
│   └── pde.rs             # PDE求解引擎（预留）
├── params/                # 参数层：参数定义与验证
│   ├── mod.rs
│   ├── common.rs          # 通用参数
│   ├── european.rs        # 欧式期权参数
│   ├── american.rs        # 美式期权参数
│   └── barrier.rs         # 障碍期权参数
├── traits/                # 核心 trait 定义
│   ├── mod.rs
│   ├── payoff.rs          # 收益函数 trait
│   ├── exercise.rs        # 行权规则 trait
│   ├── process.rs         # 随机过程 trait
│   └── engine.rs          # 引擎相关 trait
├── utils/                 # 工具层：数学工具
│   ├── mod.rs
│   ├── distributions.rs   # 分布函数
│   ├── math.rs            # 数学工具函数
│   └── linear_algebra.rs  # 线性代数工具（预留）
└── brownian/              # 随机过程模拟
    ├── mod.rs
    ├── simple.rs          # 简单布朗运动
    ├── geometric.rs       # 几何布朗运动
    └── garch.rs           # GARCH 模型
```

