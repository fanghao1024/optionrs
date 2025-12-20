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
pub trait AnalyticCalculator:Debug+Send+Sync{
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
    fn set_process(&mut self,process:Arc<dyn process::StochasticProcess>);

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
    fn set_boundary_conditions(&mut self,bc:Box<dyn BoundaryCondition>);
    fn with_new_grid_size(&self,x_steps:usize,t_steps:usize)->Result<Self>
    where Self:Sized;
    fn with_new_boundary_conditions(&self,bc:Arc<dyn BoundaryCondition>)->Result<Self>
    where Self:Sized;
}

/// PDE方法接口，类似AnalyticCalculator
pub trait PDEMethod:Debug+Send+Sync{
    /// 执行单步反向迭代
    ///
    /// # parameter
    /// - `grid`: 价值网络
    /// - `time_idx': 当前时间层索引
    /// - `s_min`: 价格下界
    /// - `dx`: 价格步长
    /// - `dt`: 时间步长
    /// - `params`: 市场参数
    /// - `payoff`: payoff函数
    /// - `exercise_rule`: 行权规则
    /// - `current_t`: 当前时点
    /// - `use_log_space`: 是否使用对数价格价格
    fn step_back(
        &self,
        grid:&mut Vec<Vec<f64>>,
        time_idx:usize,
        s_min: f64,
        dx: f64,
        dt: f64,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule,
        current_t: f64,
        use_log_space: bool
    )->Result<()>;
}
/// PDE boundary condition interface
/// PDE边界条件接口
pub trait BoundaryCondition:Debug+Send+Sync{
    /// 价格下界（S→0）的期权价值
    fn upper_boundary(&self,t:f64)->Result<f64>{return Err(OptionError::NotImplemented("BoundaryConditon:upper_boundary".to_string()));}
    /// 价格上界（S→∞）的期权价值
    fn lower_boundary(&self,t:f64)->Result<f64>{return Err(OptionError::NotImplemented("BoundaryConditon:lower_boundary".to_string()));}
    /// 终值条件（到期时T的期权价值）
    fn final_condition(&self,spot:f64)->Result<f64>{return Err(OptionError::NotImplemented("BoundaryConditon:final_condition".to_string()));}
    fn clone_box(&self) -> Box<dyn BoundaryCondition>;
}

impl Clone for Box<dyn BoundaryCondition> {
    fn clone(&self) -> Box<dyn BoundaryCondition> {
        self.clone_box()
    }
}