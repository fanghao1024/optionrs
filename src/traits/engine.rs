use crate::params::common::CommonParams;
use crate::traits::{payoff,process,exercise};
use crate::errors::*;
use crate::traits::exercise::ExerciseRule;
use crate::traits::payoff::Payoff;
use std::any::Any;
use std::fmt::Debug;

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

    /// 向下转型为Any
    fn as_any(&self) -> &dyn Any;
}

/// Engine interface supporting Greek letter calculation
/// 支持希腊字母计算的引擎接口
pub trait GreeksEngine:PriceEngine{
    /// calculate Δ
    fn delta(
        &self,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule
    ) -> Result<f64> {
        let h=0.01*params.spot();
        let params_up=params.with_spot(params.spot()+h)?;
        let params_down=params.with_spot(params.spot()-h)?;

        let price_up=self.price(&params_up,payoff,exercise_rule)?;
        let price_down=self.price(&params_down,payoff,exercise_rule)?;
        Ok((price_up-price_down)/(2.0*h))
    }

    /// calculate Γ
    fn gamma(
        &self,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule
    ) -> Result<f64> {
        let h=0.01*params.spot();
        let params_up=params.with_spot(params.spot()+h)?;
        let params_down=params.with_spot(params.spot()-h)?;
        let params_middle=params.clone();

        let price_up=self.price(&params_up,payoff,exercise_rule)?;
        let price_down=self.price(&params_down,payoff,exercise_rule)?;
        let price_middle=self.price(&params_middle,payoff,exercise_rule)?;

        Ok((price_up-2.0*price_middle+price_down)/(h*h))
    }

    /// calculate vega
    fn vega(
        &self,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule
    ) -> Result<f64> {
        let h=0.01;
        let params_up=params.with_volatility(params.volatility()+h)?;
        let params_down=params.with_volatility(params.volatility()-h)?;

        let price_up=self.price(&params_up,payoff,exercise_rule)?;
        let price_down=self.price(&params_down,payoff,exercise_rule)?;
        Ok((price_up-price_down)/(2.0*h))
    }

    /// Calculate Θ <br>
    fn theta(
        &self,
        params:&CommonParams,
        payoff:&dyn Payoff,
        exercise_rule:&dyn ExerciseRule,
    )->Result<f64>{Err(OptionError::NotImplemented("theta not implemented".to_string()))}

    /// calculate ρ
    fn rho(
        &self,
        params:&CommonParams,
        payoff:&dyn Payoff,
        exercise_rule:&dyn ExerciseRule,
    )->Result<f64>{Err(OptionError::NotImplemented("rho not implemented".to_string()))}
}

/// Monte Carlo engine specific interface <br>
/// 蒙特卡洛引擎特有接口
pub trait MonteCarloEngineExt:PriceEngine{
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
pub trait BinomialEngineExt:PriceEngine{
    fn set_steps(&mut self,steps:usize)->Result<()>;
    fn get_steps(&self)->usize;
}

/// PDE engine specific interface <br>
/// PDE引擎专属接口
pub trait PDEEngineExt:PriceEngine{
    fn set_grid_size(&mut self,x_steps:usize,t_steps:usize)->Result<()>;
    fn set_boundary_condition(&mut self,bc:Box<dyn BoundaryConditon>)->Result<()>;
}

/// PDE boundary condition interface
/// PDE边界条件接口
pub trait BoundaryConditon:Debug{
    fn upper_boundary(&self,t:f64)->f64;
    fn lower_boundary(&self,t:f64)->f64;
    fn final_condition(&self,spot:f64)->f64;
}