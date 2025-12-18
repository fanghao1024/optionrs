use std::any::Any;
use crate::errors::*;
use crate::traits::engine::{PriceEngine,GreeksEngine,BoundaryConditon};
use crate::params::common::CommonParams;
use crate::traits::{payoff::Payoff, exercise::ExerciseRule};

#[derive(Debug,Clone)]
pub struct PDEEngine{
    x_steps:usize,
    t_steps:usize,
    boundary_conditions:Option<Box<dyn BoundaryConditon>>,
}

impl PDEEngine{

}

impl PriceEngine for PDEEngine{
    fn price(&self, params: &CommonParams, payoff: &dyn Payoff, exercise_rule: &dyn ExerciseRule) -> Result<f64> {
        Ok(43.0)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BoundaryConditon for PDEEngine{
    fn clone_box(&self) -> Box<dyn BoundaryConditon> {
        Box::new(self.clone())
    }
}