use crate::params::common::CommonParams;
use crate::traits::{payoff,process,exercise};
use crate::errors::*;
use crate::traits::exercise::ExerciseRule;
use crate::traits::payoff::Payoff;

/// The interface for pricing engine <br>
/// 定价引擎接口
pub trait PriceEngine{
    /// calculate option price <br>
    /// 计算期权价格
    fn price(
        &self,
        params:&CommonParams,
        payoff:&dyn Payoff,
        exercise_rule:&dyn ExerciseRule,
    )->Result<f64>;

}

/// Engine interface supporting Greek letter calculation
/// 支持希腊字母计算的引擎接口
pub trait GreeksEngine:PriceEngine{
    /// calculate Δ
    fn delta(
        &self,
        params:&CommonParams,
        payoff:&dyn Payoff,
        exercise_rule:&dyn ExerciseRule,
    )->Result<f64>;

    /// calculate Γ
    fn gamma(
        &self,
        params:&CommonParams,
        payoff:&dyn Payoff,
        exercise_rule:&dyn ExerciseRule,
    )->Result<f64>;

    /// calculate vega
    fn vega(
        &self,
        params:&CommonParams,
        payoff:&dyn Payoff,
        exercise_rule:&dyn ExerciseRule,
    )->Result<f64>;

    /// calculate Θ
    fn theta(
        &self,
        params:&CommonParams,
        payoff:&dyn Payoff,
        exercise_rule:&dyn ExerciseRule,
    )->Result<f64>;

    /// calculate ρ
    fn rho(
        &self,
        params:&CommonParams,
        payoff:&dyn Payoff,
        exercise_rule:&dyn ExerciseRule,
    )->Result<f64>;
}

/// Monte Carlo engine specific interface <br>
/// 蒙特卡洛引擎特有接口
pub trait MonteCarloEngine:PriceEngine{
    /// Set Random process <br>
    /// 设置随机过程
    fn set_process(&mut self,process:Box<dyn process::StochasticProcess>);

    /// Set simulation number <br>
    /// 设置模拟次数
    fn set_num_simulation(&mut self,num:usize)->Result<()>;

    /// set time steps
    /// 设置时间步数
    fn set_time_steps(&mut self,time_steps:usize)->Result<()>;

}

/// Binomial engine specific interface <br>
/// 二叉树引擎专属接口
pub trait BinomialEngine:PriceEngine{
    fn set_steps(&mut self,steps:usize)->Result<()>;
}

/// PDE engine specific interface <br>
/// PDE引擎专属接口
pub trait PDEEngine:PriceEngine{
    fn set_grid_size(&mut self,x_steps:usize,t_steps:usize)->Result<()>;
    fn set_boundary_condition(&mut self,bc:Box<dyn BoundaryConditon>)->Result<()>;
}

/// PDE boundary condition interface
/// PDE边界条件接口
pub trait BoundaryConditon{
    fn upper_boundary(&self,t:f64)->f64;
    fn lower_boundary(&self,t:f64)->f64;
    fn final_condition(&self,spot:f64)->f64;
}