use std::any::Any;
use std::sync::Arc;

use crate::traits::engine::BoundaryCondition;
use crate::traits::engine::{GreeksEngine, PriceEngine};
use crate::params::common::CommonParams;
use crate::traits::{payoff::Payoff, exercise::ExerciseRule};

use super::analytic::AnalyticEngine;
use super::monte_carlo::MonteCarloEngine;
use super::binomial::BinomialEngine;
use super::pde::{PDEEngine,engine::FiniteDifferenceMethod};
use crate::errors::*;

#[derive(Debug,Clone)]
pub enum EngineConfig{
    Analytic(Arc<AnalyticEngine>),
    Binomial(Arc<BinomialEngine>),
    MonteCarlo(Arc<MonteCarloEngine>),
    PDE(Arc<PDEEngine>),
}

impl PriceEngine for EngineConfig{
    fn calculate_price(
        &self,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule
    ) -> Result<f64> {
        match self{
            EngineConfig::Analytic(engine) => {engine.calculate_price(params, payoff, exercise_rule)},
            EngineConfig::Binomial(engine) => {engine.calculate_price(params, payoff, exercise_rule)},
            EngineConfig::MonteCarlo(engine) => {engine.calculate_price(params, payoff, exercise_rule)},
            EngineConfig::PDE(engine) => {engine.calculate_price(params, payoff, exercise_rule)},
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
    pub fn monte_carlo(
        num_simulations:usize,
        time_steps:usize,
        process: Option<Arc<dyn crate::traits::process::StochasticProcess>>,
        use_antithetic:bool,
        use_parallel:bool,
        seed:u64)
        ->Result<Self>{
        Ok(
            EngineConfig::MonteCarlo(
                Arc::new(
                    MonteCarloEngine::new(
                        num_simulations,
                        time_steps,
                    process,
                    use_antithetic,
                    use_parallel,
                    seed
                    )?
                )
            )
        )
    }
    pub fn pde(
        x_steps:usize,
        t_steps:usize,
        method:FiniteDifferenceMethod,
        use_log_space:bool,
        boundary_condition:&Arc<dyn BoundaryCondition>
    )->Result<Self>{
        Ok(
            EngineConfig::PDE(
                Arc::new(
                    PDEEngine::new(
                        x_steps,
                        t_steps,
                        method,
                        use_log_space,
                        Arc::clone(boundary_condition),
                    )?
                )
            )
        )
    }
}