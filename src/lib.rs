//! 金融衍生品定价库，包含多种期权定价模型和模拟工具

// 导出所有公共模块和API
pub mod brownian;
pub mod black_scholes;
pub mod monte_carlo;
pub mod binomial;
pub mod exotic_options;
pub mod utils;
mod pde;
mod generic;

// 引入外部依赖
use rand::Rng;
use rand_distr::StandardNormal;
use statrs::distribution::{Normal, Continuous, ContinuousCDF};
use std::f64::consts::E;
use rayon::prelude::*;
use owens_t;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
