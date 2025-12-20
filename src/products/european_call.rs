use std::any::Any;
use std::sync::Arc;
use crate::core::monte_carlo::MonteCarloEngine;
use crate::simulation::brownian::GeometricBrownianMotion;
use crate::params::common::CommonParams;
use crate::errors::*;
use crate::utils::statistics::validate_common_params;
use crate::core::engine_config::EngineConfig;
use crate::traits::payoff::CallPayoff;
use crate::traits::exercise::{EuropeanExercise,ExerciseRule};



#[derive(Clone)]
pub struct EuropeanCall{
    common:CommonParams,
    payoff:CallPayoff,
    exercise_type:Arc<dyn ExerciseRule>
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
        let payoff=CallPayoff{strike:strike};
        let european_exercise=EuropeanExercise;
        validate_common_params(&common)?;
        Ok(Self{
            common,
            payoff,
            exercise_type:Arc::new(european_exercise)
        })
    }

    pub fn common(&self)->&CommonParams{&self.common}
    pub fn payoff(&self)->&CallPayoff{&self.payoff}
    pub fn exercise_type(&self)->&dyn ExerciseRule{(&self.exercise_type).as_ref()}
    pub fn price_params(&self)->(
        &CommonParams,
        &CallPayoff,
        &Arc<dyn ExerciseRule>)
    {
        (&self.common,&self.payoff,&self.exercise_type)
    }
}