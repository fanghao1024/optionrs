use crate::params::common::CommonParams;
use crate::traits::{payoff,process,exercise};
use crate::errors::*;
use crate::traits::exercise::ExerciseRule;
use crate::traits::payoff::Payoff;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

/// The interface for pricing engine <br>
/// 定价引擎接口
pub trait PriceEngine:Send+Sync{
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

/// 解析解计算器插件Trait(核心：插件化的核心契约）
pub trait AnalyticCalculator:Send+Sync{
    fn supported_types(&self)->Vec<payoff::AnalyticPayoffType>;

    /// 计算解析解价格（插件核心逻辑）
    /// 参数：Payoff(含专属参数）+通用参数
    fn calculate(&self, params:&CommonParams, payoff:&dyn Payoff)->Result<f64>;
}

/// 类型别名
pub type AnalyticCalculatorRef = Arc<dyn AnalyticCalculator>;

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
    fn upper_boundary(&self,t:f64)->Result<f64>{return Err(OptionError::NotImplemented("BoundaryConditon:upper_boundary".to_string()));}
    fn lower_boundary(&self,t:f64)->Result<f64>{return Err(OptionError::NotImplemented("BoundaryConditon:lower_boundary".to_string()));}
    fn final_condition(&self,spot:f64)->Result<f64>{return Err(OptionError::NotImplemented("BoundaryConditon:final_condition".to_string()));}
    fn clone_box(&self) -> Box<dyn BoundaryConditon>;
}

impl Clone for Box<dyn BoundaryConditon> {
    fn clone(&self) -> Box<dyn BoundaryConditon> {
        self.clone_box()
    }
}