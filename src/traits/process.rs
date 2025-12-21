use crate::errors::*;
use std::fmt::Debug;

/// Random process interface
/// 随机过程接口
pub trait StochasticProcess:Debug+Send+Sync{
    ///To solve dyn Clone problem
    fn clone_box(&self) -> Box<dyn StochasticProcess>;
    /// Initialize the random generator
    /// 初始化随机生成器
    fn init_rng_with_seed(&mut self,seed:u64);

    /// Simulate the price for the next time step
    /// 模拟下一个时间步的价格
    /// ## parameters
    /// + current_price: 当前价格
    /// + time_step: time step(year) 时间步长（年）
    fn next_step(&mut self,current_price:f64,time_step:f64)->Result<f64>;

    /// Simulate the complete path
    /// 模拟完整路径
    /// ## parameters
    /// + initial_price: 初始价格
    /// + time_horizon: total time(year) 总时间（年）
    /// + steps: 步数
    fn simulate_path(
        &mut self,
        initial_price:f64,
        time_horizon:f64,
        steps:usize
    )->Result<Vec<f64>>{
        Err(OptionError::NotImplemented("simulate_path not implemented".to_string()))
    }

    fn simulate_antithetic_path(
        &mut self,
        initial_price:f64,
        time_horizon:f64,
        steps:usize,
    )->Result<(Vec<f64>,Vec<f64>)>{Err(OptionError::NotImplemented("Simulate antithetic_path function not implemented".into()))}
}

impl Clone for Box<dyn StochasticProcess> {
    fn clone(&self) -> Box<dyn StochasticProcess> {
        self.clone_box()
    }
}