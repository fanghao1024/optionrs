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

## Usage
Simply implement the market parameters,payoff function, exercise type and boundary condition for
the option in the src/products/ directory, and the pricing engine
can be use to complete.
### Taking European call option as example
+ option parameters
```rust
#[derive(Clone)]
pub struct EuropeanCall{
    common:CommonParams,
    strike:f64,
    payoff:CallPayoff,
    exercise_type:Arc<dyn ExerciseRule>,
    boundary_condition:Arc<dyn BoundaryCondition>,

}
```
+ payoff function
```rust
#[derive(Debug,Clone,Copy)]
pub struct CallPayoff{
  pub strike:f64,
}
impl CallPayoff {
  pub fn new(strike:f64)->Self{
    Self{strike}
  }
}
impl Payoff for CallPayoff{
  fn payoff(&self,spot:f64)->f64{
    (spot-self.strike).max(0.0)
  }
  fn as_any(&self) -> &dyn Any {
    self
  }
  fn analytic_type(&self)->Option<AnalyticPayoffType>{
    Some(AnalyticPayoffType::VanillaCall)
  }
}
```
**which is in use crate::traits::payoff directory**

+ exercise type
The definition of the rule is in src/traits/exercise
**EuropeanExercise** and **AmericanExercise**

+ boundary condition
```rust
#[derive(Debug,Clone)]
pub struct CallBoundaryCondition{
    strike:f64,
    risk_free_rate:f64,
    volatility:f64,
}

impl CallBoundaryCondition{
    pub fn new(
        strike:f64,
        risk_free_rate:f64,
        volatility:f64,
    )->Result<Self>{
        if strike<0.0{
            return Err(OptionError::InvalidParameter("Strike cannot be negative".to_string()));
        }
        Ok(Self{strike, risk_free_rate, volatility})
    }
}

impl BoundaryCondition for CallBoundaryCondition{
    /// price lower boundary（ when S → 0）
    fn lower_boundary(&self, _t: f64) -> Result<f64> {
        Ok(0.0)
    }

    /// price upper boundary（when S → ∞）
    fn upper_boundary(&self, t: f64) -> Result<f64> {
        let discount_factor=(-self.risk_free_rate*t).exp();
        let k=4.0;
        let s_max=self.strike*(k*self.volatility*t.sqrt()).exp();
        Ok(s_max-self.strike*discount_factor)
    }
  
    /// final condition when time reaches maturity
    fn final_condition(&self, spot: f64) -> Result<f64> {
        Ok((spot-self.strike).max(0.0))
    }

    fn clone_box(&self) -> Box<dyn BoundaryCondition> {
        Box::new(self.clone())
    }
}
```
+ example
```rust

use optionrs::prelude::*;
use optionrs::products::european_call::EuropeanCall;

fn main() ->Result<()>{
    // European Call S=100, r=5%, σ=20%, q=0, t=1 year
    let european_call=EuropeanCall::new(
        100.0,
        100.0,
        0.05,
        0.2,
        0.0,
        1.0
    )?;

    // Binomial tree pricing model
    let binomial_engine=EngineConfig::binomial(800)?;
    let param_price=binomial_engine.price(&european_call)?;
    println!("Binomial price: {param_price}");


    // Monte Carlo pricing model
    let mc_engine:EngineConfig=EngineConfig::monte_carlo(
        100_000,
        100,
        Some(Arc::new(GeometricBrownianMotion::new(
            0.05,
            0.2
        )?)),
        false,
        true,
        0
    )?;
    let mc_price=mc_engine.price(&european_call)?;
    println!("Monte Carlo engine price: {mc_price}");

    // analytic solution of European option by Black-Scholes
    let analytic_engine=EngineConfig::analytic()?;
    let analytic_price=analytic_engine.price(&european_call)?;
    println!("Analytic engine price: {analytic_price}");


    // pde infinite differential method: Explicit
    let pde_engine=EngineConfig::pde(
        200,
        600,FiniteDifferenceMethod::Explicit,
        true,
        european_call.boundary_condition()
    )?;
    let pde_price=pde_engine.price(&european_call)?;
    println!("PDE Explicit price: {pde_price}");

    // pde infinite differential method: Implicit
    let pde_engine=EngineConfig::pde(
        200,
        600,FiniteDifferenceMethod::Implicit,
        true,
        european_call.boundary_condition()
    )?;
    let pde_price=pde_engine.price(&european_call)?;
    println!("PDE Implicit price: {pde_price}");

    // pde infinite differential method: Crank-Nicolson method
    let pde_engine=EngineConfig::pde(
        200,
        600,FiniteDifferenceMethod::CrankNicolson,
        true,
        european_call.boundary_condition()
    )?;
    let pde_price=pde_engine.price(&european_call)?;
    println!("PDE crank-nicolson price: {pde_price}");


    Ok(())
}
```
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
│   ├── pde.rs             
│   ├── pde/                     # PDE求解引擎
│   │   ├── mod.rs               
│   │   └── methods/
│   │       ├── mod.rs
│   │       ├── explicit.rs
│   │       ├── implicit.rs
│   │       └── crank_nicolson.rs
│   ├── binomial.rs        # 二叉树引擎
│   ├── monte_carlo.rs     # 蒙特卡洛引擎
│   ├── analytic/           # 解析解引擎核心
│   │   ├── mod.rs
│   │   ├── engine.rs       # 插件化AnalyticEngine（计算器注册表）
│   │   └── calculators/    # 各类解析解计算器（插件）
│   │       ├── mod.rs
│   │       ├── vanilla.rs  # 普通期权计算器
│   │       ├── binary.rs   # 二元期权计算器
│   │       └── barrier.rs  # 障碍期权计算器
│   └── engine_config.rs    # 所有引擎的统一入口枚举 
├── params/                # 参数层：参数定义与验证
│   ├── mod.rs
│   ├── common.rs          # 通用参数
│   ├── european.rs        # 欧式期权参数
│   ├── american.rs        # 美式期权参数
│   └── barrier.rs         # 障碍期权参数
├── traits/                # 抽象接口层（仅定义Trait，无实现）
│   ├── mod.rs
│   ├── payoff.rs          # Payoff抽象+解析解类型枚举
│   ├── exercise.rs        # 行权规则抽象 trait
│   ├── process.rs         # 随机过程 trait
│   └── engine.rs          # 定价引擎+解析解计算器插件Trait
├── utils/                 # 工具层：数学工具
│   ├── mod.rs
│   ├── statistics.rs      # 正态分布CDF/PDF、参数校验
│   ├── math.rs            # 数学工具函数
│   └── linear_algebra.rs  # 线性代数工具（预留）
└── brownian/              # 随机过程模拟
    ├── mod.rs
    ├── simple.rs          # 简单布朗运动
    ├── geometric.rs       # 几何布朗运动
    └── garch.rs           # GARCH 模型
```

