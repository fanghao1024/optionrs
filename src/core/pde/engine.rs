//! PDE pricing engine

use std::any::Any;
use super::methods::{ ExplicitMethod, ImplicitMethod, CrankNicolsonMethod};
use std::sync::Arc;
use crate::traits::engine::{PriceEngine, PDEMethod, PDEEngineExt, BoundaryCondition};
use crate::params::common::CommonParams;
use crate::errors::*;
use crate::traits::{payoff::Payoff,exercise::ExerciseRule};
use crate::utils::math::linear_interpolate;

/// PDE方法类型枚举
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum FiniteDifferenceMethod{
    Explicit,
    Implicit,
    CrankNicolson,
}

/// PDE引擎配置
#[derive(Debug,Clone)]
pub struct PDEEngine{
    pub x_steps:usize,
    pub t_steps:usize,
    pub method:FiniteDifferenceMethod,
    pub use_log_space:bool,
    boundary_condition:Arc<dyn BoundaryCondition>,
    method_instance:Arc<dyn PDEMethod>,
}

impl PDEEngine{
    pub fn new(
        x_steps:usize,
        t_steps:usize,
        method:FiniteDifferenceMethod,
        use_log_space:bool,
        boundary_condition:Arc<dyn BoundaryCondition>,
    )->Result<Self>{
        if x_steps < 50 || t_steps < 50 {
            return Err(OptionError::InvalidParameter("The steps of PDE grids cannot \
            be less than 50 steps (recommeng ≥ 200)".to_string()));
        }
        if use_log_space && (x_steps<100 || t_steps<100) {
            return Err(OptionError::InvalidParameter("Log space method recommends steps greater than 100".to_string()))
        }
        let method_instance:Arc<dyn PDEMethod>=match method{
            FiniteDifferenceMethod::Explicit => Arc::new(ExplicitMethod::new()),
            FiniteDifferenceMethod::Implicit => Arc::new(ImplicitMethod::new()),
            FiniteDifferenceMethod::CrankNicolson => Arc::new(CrankNicolsonMethod::new()),
        };
        Ok(Self{
            x_steps,
            t_steps,
            method,
            use_log_space,
            boundary_condition,
            method_instance,
        })
    }

}

impl PriceEngine for PDEEngine{
    fn calculate_price(&self, params: &CommonParams, payoff: &dyn Payoff, exercise_rule: &dyn ExerciseRule) -> Result<f64> {
        let s0=params.spot();
        let t_total=params.time_to_maturity();
        let sigma=params.volatility();

        let (s_min,s_max,s_current,to_price):(f64,f64,f64,fn(f64)->f64)=if self.use_log_space{

            ((0.1*s0).ln(),(2.0*s0).ln(),s0.ln(),|s:f64|s.exp())
        }else{

            (0.1*s0,2.0*s0,s0,|s:f64| s)
        };
        let dx=(s_max-s_min)/self.x_steps as f64;
        let dt=t_total/self.t_steps as f64;

        // 稳定性检查（仅显式法需要）
        // 显式有限差分法的稳定性通常由 CFL 条件（Courant-Friedrichs-Lewy Condition） 决定
        if matches!(self.method,FiniteDifferenceMethod::Explicit){
            let stability_factor=if self.use_log_space{
                sigma.powi(2)*dt/dx.powi(2)
            }else{
                sigma.powi(2)*(2.0*s0).powi(2)*dt/dx.powi(2)
            };
            if stability_factor > 0.5 {
                return Err(OptionError::Other(format!(
                    "显式法稳定性条件不满足：factor={:.3} > 0.5，\n\
                    建议：\n\
                    1. 增加价格网格步数（当前{}）\n\
                    2. 增加时间网格步数（当前{}）\n\
                    3. 改用隐式法或Crank-Nicolson",
                    stability_factor, self.x_steps, self.t_steps
                )));
            }
        }

        // 初始化网格
        let mut grid=vec![vec![0.0;self.x_steps+1];self.t_steps+1];

        // 终值条件
        for i in 0..=self.x_steps{
            let s_space=s_min+i as f64 *dx;
            let s=to_price(s_space);
            grid[self.t_steps][i]=payoff.payoff(s);
        }

        for n in (0..self.t_steps).rev(){
            let current_t=n as f64 * dt;
            let remaining_time=t_total-current_t;

            //边界条件
            grid[n][0]=self.boundary_condition.lower_boundary(remaining_time)?;
            grid[n][self.x_steps]=self.boundary_condition.upper_boundary(remaining_time)?;

            self.method_instance.step_back(
                &mut grid,
                n,
                s_min,
                dx,
                dt,
                params,
                payoff,
                exercise_rule,
                current_t,
                self.use_log_space
            )?;
        }

        let price=linear_interpolate(s_current,s_min,dx,&grid[0])?.max(0.0);
        Ok(price)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl PDEEngineExt for PDEEngine{
    fn set_grid_size(&mut self, x_steps: usize, t_steps: usize) -> Result<()> {
        if x_steps<50 || t_steps<50{
            return Err(OptionError::InvalidParameter("The steps of PDE grids cannot \
            be less than 50 steps (recommeng ≥ 200)".to_string()));
        }
        self.x_steps=x_steps;
        self.t_steps=t_steps;
        Ok(())
    }

    fn set_boundary_conditions(&mut self, bc: Box<dyn BoundaryCondition>) {
        self.boundary_condition=Arc::from(bc);
    }

    fn with_new_grid_size(&self, x_steps: usize, t_steps: usize) -> Result<Self>
    where
        Self: Sized,
    {
        let mut new=self.clone();
        new.set_grid_size(x_steps, t_steps)?;
        Ok(new)
    }

    fn with_new_boundary_conditions(&self, bc: Arc<dyn BoundaryCondition>) -> Result<Self>
    where
        Self: Sized,
    {
        let mut new=self.clone();
        new.boundary_condition=bc;
        Ok(new)
    }
}


unsafe impl Send for PDEEngine{}
unsafe impl Sync for PDEEngine{}