//! explicit 显式法
use crate::traits::engine::PDEMethod;
use crate::params::common::CommonParams;
use crate::traits::{payoff::Payoff,exercise::ExerciseRule,engine::BoundaryCondition};

#[derive(Debug,Clone)]
pub struct ExplicitMethod;

impl ExplicitMethod{
    pub fn new()->Self{
        Self
    }
}

impl PDEMethod for ExplicitMethod{
    fn step_back(
        &self,
        grid_next: &[f64],
        grid_current: &mut [f64],
        s_min: f64,
        dx: f64,
        dt: f64,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule,
        boundary_condition: &dyn BoundaryCondition,
        current_t: f64
    ) -> crate::Result<()> {
        todo!()
    }
}