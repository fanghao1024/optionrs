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
