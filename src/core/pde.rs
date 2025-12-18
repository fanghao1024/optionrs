use std::any::Any;
use std::sync::Arc;
use crate::errors::*;
use crate::traits::engine::{PriceEngine,GreeksEngine,BoundaryConditon};
use crate::params::common::CommonParams;
use crate::traits::{payoff::Payoff, exercise::ExerciseRule};

#[derive(Debug,Clone)]
pub struct PDEEngine{
    x_steps:usize,
    t_steps:usize,
    boundary_conditions:Arc<dyn BoundaryConditon>,
}

impl PDEEngine{
    pub fn new(
        x_steps:usize,
        t_steps:usize,
        boundary_conditions:Arc<dyn BoundaryConditon>,
    ) -> Result<Self>{
        if x_steps<50 || t_steps<50{
            return Err(OptionError::InvalidParameter("The steps of PDE grids cannot be less than 50 steps".to_string()));
        }
        Ok(Self{
            x_steps,
            t_steps,
            boundary_conditions,
        })
    }
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