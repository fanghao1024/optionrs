//! PDE pricing engine

use std::ops::Bound;
use super::methods::{FiniteDifferenceMethod, ExplicitMethod, ImplicitMethod, CrankNicolsonMethod};
use std::sync::Arc;
use crate::core::pde::methods::FiniteDifferenceMethod::CrankNicolson;
use crate::traits::engine::{PriceEngine, PDEMethod, PDEEngineExt, BoundaryCondition};
use crate::params::common::CommonParams;
use crate::errors::*;
use crate::traits::{payoff::Payoff,exercise::ExerciseRule};


/// PDE引擎配置
#[derive(Debug,Clone)]
pub struct PDEEngine{
    pub x_step:usize,
    pub t_step:usize,
    pub method:FiniteDifferenceMethod,
    pub use_log_space:bool,
    boundary_condition:Arc<dyn BoundaryCondition>,
    method_instance:Arc<dyn PDEMethod>,
}

impl PDEEngine{
    pub fn new(
        x_step:usize,
        t_step:usize,
        method:FiniteDifferenceMethod,
        use_log_space:bool,
        boundary_condition:Arc<dyn BoundaryCondition>,
    )->Result<Self>{
        if x_step < 50 || t_step < 50 {
            return Err(OptionError::InvalidParameter("The steps of PDE grids cannot \
            be less than 50 steps (recommeng ≥ 200)".to_string()));
        }
        if use_log_space && (x_step<100 || t_step<100) {
            return Err(OptionError::InvalidParameter("Log space method recommends steps greater than 100".to_string()))
        }
        let method_instance=match method{
            FiniteDifferenceMethod::Explicit => Arc::new(ExplicitMethod::new()),
            FiniteDifferenceMethod::Implicit => Arc::new(ImplicitMethod::new()),
            FiniteDifferenceMethod::CrankNicolson => Arc::new(CrankNicolsonMethod::new()),
        };
        Ok(Self{
            x_step,
            t_step,
            method,
            use_log_space,
            boundary_condition,
            method_instance,
        })
    }

    fn calculate_price_impl(
        &self,
        params:&CommonParams,
        payoff:&dyn Payoff,
        exercise:&dyn ExerciseRule
    )->Result<f64>{
        let (s0,r,sigma,q,t_total)=params.all_params();

    }
}


