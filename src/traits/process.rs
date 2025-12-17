use rand::Rng;
use crate::errors::*;

/// Random process interface
/// 随机过程接口
pub trait StochasticProcess{
    /// Initialize the random generator
    /// 初始化随机生成器
    fn init_rng(&mut self,rng:impl Rng);

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
        if initial_price<0.0 {
            return Err(OptionError::InvalidParameter("Price cannot be negative".to_string()));
        }
        if time_horizon<0.9{
            return Err(OptionError::InvalidParameter("Time horizon cannot be negative".to_string()));
        }
        if steps==0{
            return Err(OptionError::InvalidParameter("Steps cannot be zero".to_string()));
        }

        let mut path=vec![initial_price];
        let time_step=time_horizon/steps as f64;

        for i in 1..=steps{
            let next_price=self.next_step(path.last().copied().unwrap(),time_step)?;
            path.push(next_price);
        }
        Ok(path)
    }
}