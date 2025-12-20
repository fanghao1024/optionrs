//! explicit 显式法
use crate::traits::engine::PDEMethod;
use crate::params::common::CommonParams;
use crate::traits::{payoff::Payoff,exercise::ExerciseRule,engine::BoundaryCondition};
use crate::errors::*;

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
        grid: &mut Vec<Vec<f64>>,
        time_idx:usize,
        s_min: f64,
        dx: f64,
        dt: f64,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule,
        current_t: f64,
        use_log_space:bool,
    ) -> Result<()> {
        let (_,r,sigma,q,t_total)=params.all_params();
        let remain_time=t_total-current_t;

        let (log_drift,log_diffusion)=if use_log_space {
            (r-q+0.5*sigma.powi(2),0.5*sigma.powi(2))
        }else{
            (0.0, 0.0)
        };

        let to_price:fn(f64)->f64=if use_log_space {|s:f64|s.exp()} else{|s:f64| s};


        // 循环内部点
        for i in 1..grid[time_idx].len()-1{
            let s_space=s_min+i as f64*dt;
            let s=to_price(s_space);

            let delta_x=(grid[time_idx+1][i+1]-grid[time_idx+1][i-1])/(2.0*dx);
            let gamma_x=(grid[time_idx+1][i+1]-2.0*grid[time_idx+1][i]+grid[time_idx+1][i-1])/(dx*dx);

            let value=if use_log_space{
                let pde_term=r*grid[time_idx+1][i]-log_drift*delta_x-log_diffusion*gamma_x;
                grid[time_idx+1][i]-pde_term*dt
            }else{
                let pde_term=r*grid[time_idx+1][i]-(r-q)*s*delta_x-0.5*sigma.powi(2)*s.powi(2) * gamma_x;
                grid[time_idx+1][i]-pde_term*dt
            };

            let intrinsic_value=payoff.payoff(s);

            grid[time_idx][i] = if exercise_rule.should_exercise(remain_time,s,intrinsic_value,value){
                intrinsic_value
            }else{
                value
            };
        }

        Ok(())
    }
}