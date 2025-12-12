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
optionpricing/          
├── Cargo.toml          # Dependency/Configuration Files 依赖/配置文件
├── README.md           
├── src/                
│   ├── lib.rs
│   ├── black_scholes/mod.rs # Pricing Methods for European Options 欧式期权定价相关方法
│   │   ├──european_call      # European call 欧式看涨
│   │   ├──european_put       # European put  欧式看跌
│   │   ├──black_scholes_call_delta      # European call's Greek letter delta 欧式看涨的希腊字母Δ
│   │   ├──black_scholes_call_gamma      # European call's Greek letter gamma 欧式看涨期权的希腊字母Γ
│   │   └──black_scholes_call_implied_vol      # European call's implied volatility 欧式看涨期权的隐含波动率
│   ├──brownian/mod.rs # Simulate browian motions
│   │   ├──simulating_brown_motion      #Simulaing brown motion 模拟布朗运动
│   │   └──simulating_geo_brown_motion      #Simulate geometric Brownian motion 模拟几何布朗运动
│   ├──binomial      #binomial tree pricing 二叉树定价
│   │   ├──euporean_call_binomial      # 欧式看涨期权二叉树定价 
│   │   ├──american_put_binomial       # 美式看跌期权二叉树定价
│   │   ├──american_put_binomial_delta_gamma      # 美式看跌期权二叉树定价（含Delta和Gamma）
│   │   ├──american_spread_put_binomial      # 美式价差看跌期权二叉树定价
│   │   ├──american_put_binomial_bs          # 美式看跌期权二叉树定价（Black-Scholes调整）
│   │   └──american_put_binomial_bs_re       # 美式看跌期权二叉树定价（Black-Scholes调整，Richardson外推）
│   ├──monte_carlo      # 蒙特卡洛模拟定价
│   │   ├──european_call_mc      # 欧式看涨期权蒙特卡洛定价（含Delta估计）
│   │   ├──european_call_garch_mc      # GARCH模型下欧式看涨期权蒙特卡洛定价
│   │   ├──floating_striking_call_mc_se      # 浮动执行价回望看涨期权蒙特卡洛定价（含标准误差）
│   │   ├──european_basket_call_mc      # 欧式篮子看涨期权蒙特卡洛定价
│   │   ├──floating_striking_call_mc_av_se      # 使用对偶变量法的浮动执行价回望看涨期权蒙特卡洛定价（含标准误差）
│   │   └──average_price_call_mc      # 用几何平均价格作为控制变量，定价含过去价格的算术平均价格看(亚式期权)
│   ├──pde
│   │   ├──european_call_crank_nicolson      # 欧式看涨期权Crank-Nicolson定价函数
│   │   └──down_and_out_call_crank_nicolson      # 向下敲出（Down-and-Out）看涨期权Crank-Nicolson定价函数
│   ├──generic      # 远期、期货和互换期权定价公式
│   │   ├──generic_option      # 通用期权定价公式（基础公式）
│   │   ├──margrabe       # margrabe期权定价（资产交换期权）
│   │   ├──black_call     # Black模型看涨期权定价（基于远期价格）
│   │   ├──black_Put      # Black模型看跌期权定价
│   │   ├──margrabe_deferred      # 延迟生效margrabe期权
│   │   ├──black_call_2      # Black模型看涨期权（重载实现）
│   │   ├──black_call_implied_vol      # Black模型看涨期权隐含波动率
│   │   └──black_call_delta      # Black模型看涨期权Delta
│   ├──exotic_options      # 奇异期权
│   │   ├──forward_start_call      # 远期生效看涨期权
│   │   ├──down_and_out_call       # 向下敲出看涨期权
│   │   ├──floating_strike_call    # 浮动执行价格回望看涨期权
│   │   ├──discrete_geom_average_price_call      # 离散抽样几何平均价格看涨期权
│   │   ├──call_on_call      # 看涨期权上的看涨期权
│   │   ├──call_on_put       # 看跌期权上的看涨期权
│   │   ├──american_call_dividened      # 含红利的美式看涨期权
│   │   ├──chooser      # 选择期权
│   │   └──call_on_max      # 最大值看涨期权
│   ├──utils      # 通用工具函数
│   │   ├──bivariate_standard_normal_cdf      # 二元正态分布累计函数
│   │   ├──calc_percentage      # 计算数组的指定百分位数
│   │   ├──Simulated_Delta_Hedge_Profit_Forward      # 模拟Delta对冲策略的利润，并返回指定百分位数
│   │   ├──cholesky      # Cholesky 分解（乔列斯基分解）
│   │   ├──cholesky_vec      # Cholesky分解函数的重载
│   │   └──crank_nicolson      # Crank-Nicolson算法核心函数
│   └── ...
└── tests/              # 集成测试目录
    └── pricing_consistency.rs
```

