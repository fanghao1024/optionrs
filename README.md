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
optionpricing/          # crate 根目录
├── Cargo.toml          # 依赖/配置文件
├── README.md           # 核心说明文档（必须放这里）
├── src/                # 源码目录
│   ├── lib.rs
│   │   ├──
│   ├── black_scholes/mod.rs
│   └── ...
└── tests/              # 集成测试目录
    └── pricing_consistency.rs
```

