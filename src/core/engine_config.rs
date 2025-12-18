use std::any::Any;
use std::sync::Arc;

use crate::traits::engine::{GreeksEngine, PriceEngine};
use crate::params::common::CommonParams;
use crate::traits::{payoff::Payoff, exercise::ExerciseRule};

use super::analytic::AnalyticEngine;
use super::monte_carlo::MonteCarloEngine;
use super::binomial::BinomialEngine;
use super::pde::PDEEngine;
use crate::errors::*;

#[derive(Debug,Clone)]
pub enum EngineConfig{
    Analytic(Arc<AnalyticEngine>),
    Binomial(Arc<BinomialEngine>),
    MonteCarlo(Arc<MonteCarloEngine>),
    PDE(Arc<PDEEngine>),
}

impl PriceEngine for EngineConfig{
    fn price(
        &self,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule
    ) -> Result<f64> {
        match self{
            EngineConfig::Analytic(engine) => {engine.price(params, payoff, exercise_rule)},
            EngineConfig::Binomial(engine) => {engine.price(params, payoff, exercise_rule)},
            EngineConfig::MonteCarlo(engine) => {engine.price(params, payoff, exercise_rule)},
            EngineConfig::PDE(engine) => {engine.price(params, payoff, exercise_rule)},
        }
    }

    fn as_any(&self) -> &dyn Any {
        match self {
            EngineConfig::Analytic(engine) => engine.as_any(),
            EngineConfig::MonteCarlo(engine)=>engine.as_any(),
            EngineConfig::Binomial(engine)=>engine.as_any(),
            EngineConfig::PDE(engine)=>engine.as_any(),
        }
    }
}

impl EngineConfig{
    pub fn default_analytic()->Result<Self>{
        Ok(EngineConfig::Analytic(Arc::new(AnalyticEngine::default())))
    }
    pub fn binomial(steps:usize)->Result<Self>{
        Ok(EngineConfig::Binomial(Arc::new(BinomialEngine::new(steps)?)))
    }
    pub fn monte_carlo(num_simulations:usize,time_steps:usize)->Result<Self>{
        Ok(EngineConfig::MonteCarlo(Arc::new(MonteCarloEngine::new(num_simulations,time_steps)?)))
    }
    pub fn pde(x_steps:usize,t_steps:usize)->Result<Self>{
        Ok(EngineConfig::PDE(Arc::new(PDEEngine::new(x_steps,t_steps)?)))
    }
}