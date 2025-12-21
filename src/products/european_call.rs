use std::sync::Arc;
use crate::params::common::CommonParams;
use crate::errors::*;
use crate::utils::statistics::validate_common_params;
use crate::traits::payoff::{CallPayoff,Payoff};
use crate::traits::exercise::{EuropeanExercise,ExerciseRule};
use crate::traits::engine::{BoundaryCondition,PricingTrait};



#[derive(Clone)]
pub struct EuropeanCall{
    common:CommonParams,
    strike:f64,
    payoff:CallPayoff,
    exercise_type:Arc<dyn ExerciseRule>,
    boundary_condition:Arc<dyn BoundaryCondition>,

}

impl EuropeanCall{
    pub fn new(
        spot:f64,
        strike:f64,
        risk_free_rate:f64,
        volatility:f64,
        dividend_yield:f64,
        time_to_maturity:f64,
    )->Result<Self>{
        let common=CommonParams::new(
            spot,
            risk_free_rate,
            volatility,
            dividend_yield,
            time_to_maturity,
        )?;
        validate_common_params(&common)?;
        let payoff=CallPayoff{strike:strike};
        let european_exercise=EuropeanExercise;
        let boundary_condition=CallBoundaryCondition::new(strike,risk_free_rate,volatility)?;

        Ok(Self{
            common,
            strike,
            payoff,
            exercise_type:Arc::new(european_exercise),
            boundary_condition:Arc::new(boundary_condition)
        })
    }

    pub fn condition(&self)->Result<(
        &CommonParams,
        impl Payoff,
        impl ExerciseRule,
        impl BoundaryCondition
    )>{
        Ok((
            &self.common,
            CallPayoff::new(self.strike),
            EuropeanExercise::new(),
            CallBoundaryCondition::new(self.strike,self.common.risk_free_rate(),self.common.volatility())?
        ))
    }
}

impl PricingTrait for EuropeanCall{
    fn common(&self) -> &CommonParams { &self.common }
    fn payoff(&self)->&dyn Payoff{&self.payoff}
    fn exercise_type(&self)->&dyn ExerciseRule{(&self.exercise_type).as_ref()}
    fn boundary_condition(&self)->&Arc<dyn BoundaryCondition>{&self.boundary_condition}
}
/// Boundary Condition config
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
    /// 价格下界（S → 0）
    fn lower_boundary(&self, _t: f64) -> Result<f64> {
        Ok(0.0)
    }

    /// 价格上界（S → ∞）
    fn upper_boundary(&self, t: f64) -> Result<f64> {
        // S→∞时，C ≈ S - K*e^(-rt)，这里用 S_max = 2K 做近似
        // 或者也可以使用动态边界条件
        // 基于波动率动态设置：S_max = K * exp(κ * σ * sqrt(T))
        // κ通常取3-5，覆盖99.9%以上的概率质量
        let discount_factor=(-self.risk_free_rate*t).exp();
        //Ok(2.0*self.strike-self.strike*discount_factor)
        let k=4.0;
        let s_max=self.strike*(k*self.volatility*t.sqrt()).exp();
        Ok(s_max-self.strike*discount_factor)
    }

    fn final_condition(&self, spot: f64) -> Result<f64> {
        Ok((spot-self.strike).max(0.0))
    }

    fn clone_box(&self) -> Box<dyn BoundaryCondition> {
        Box::new(self.clone())
    }
}
