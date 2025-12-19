use std::any::Any;
use std::sync::Arc;
use crate::errors::*;
use crate::traits::engine::{PriceEngine,GreeksEngine,BoundaryCondition,PDEEngineExt};
use crate::params::common::CommonParams;
use crate::traits::{payoff::Payoff, exercise::ExerciseRule};

#[derive(Debug,Clone)]
pub struct PDEEngine{
    x_steps:usize,
    t_steps:usize,
    boundary_conditions:Arc<dyn BoundaryCondition>,
}

impl PDEEngine{
    pub fn new(
        x_steps:usize,
        t_steps:usize,
        boundary_conditions:Arc<dyn BoundaryCondition>,
    ) -> Result<Self>{
        if x_steps<50 || t_steps<50{
            return Err(OptionError::InvalidParameter("The steps of PDE grids cannot \
            be less than 50 steps (recommeng ≥ 200)".to_string()));
        }
        Ok(Self{
            x_steps,
            t_steps,
            boundary_conditions,
        })
    }

    /// 修改网格步数
    pub fn with_grid_size(self,x_steps:usize,t_steps:usize) -> Result<Self>{
        if x_steps<50 || t_steps<50{
            return Err(OptionError::InvalidParameter("The steps of PDE grids cannot \
            be less than 50 steps (recommeng ≥ 200)".to_string()));
        }
        Ok(Self{
            x_steps,
            t_steps,
            boundary_conditions: self.boundary_conditions.clone(),
        })
    }

    /// 修改边界条件
    pub fn with_boundary_conditions(self,bc:Arc<dyn BoundaryCondition>)->Result<Self>{
        Ok(
            Self{
                x_steps:self.x_steps,
                t_steps:self.t_steps,
                boundary_conditions: bc,
            }
        )
    }

    /// 内部工具：线性插值
    fn linear_interpolate(&self,s0:f64,s_min:f64,dx:f64,grid:&[f64])->f64{
        if s0<=s_min{
            return grid[0];
        }
        let i_float=(s0-s_min)/dx;
        let grid_len=grid.len();

        if i_float>=(grid.len()-1) as f64{
            return grid[grid_len-1];
        }

        let i_floor=i_float as usize;
        let i_ceil=i_floor+1;

        let weight=i_float-i_floor as f64;

        grid[i_floor]+(grid[i_ceil]-grid[i_floor])*weight
    }
}

impl PDEEngineExt for PDEEngine{
    /// 单线程可用
    fn set_grid_size(&mut self, x_steps: usize, t_steps: usize) -> Result<()> {
        if x_steps<50 || t_steps<50{
            return Err(OptionError::InvalidParameter("The steps of PDE grids cannot \
            be less than 50 steps (recommeng ≥ 200)".to_string()));
        }
        self.x_steps = x_steps;
        self.t_steps = t_steps;
        Ok(())
    }
    /// 单线程可用
    fn set_boundary_conditions(&mut self, bc:Box<dyn BoundaryCondition>) {
        self.boundary_conditions=Arc::from(bc);
    }

    fn with_new_grid_size(&self,x_steps:usize,t_steps:usize)->Result<Self>
    where Self:Sized{
        self.clone().with_grid_size(x_steps, t_steps)
    }

    fn with_new_boundary_conditions(&self, bc: Arc<dyn BoundaryCondition>) -> Result<Self>
    where
        Self: Sized,
    {
        self.clone().with_boundary_conditions(bc)
    }
}

impl PriceEngine for PDEEngine{
    fn price(
        &self,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule
    ) -> Result<f64> {
        let (s0,r,q,sigma,t_total)=params.all_params();

        let s_min=0.1*s0;

    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BoundaryCondition for PDEEngine{
    fn clone_box(&self) -> Box<dyn BoundaryCondition> {
        Box::new(self.clone())
    }
}