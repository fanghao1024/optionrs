use crate::errors::*;
use crate::ExerciseRule;
use crate::traits::engine::{PriceEngine, PDEEngineExt, PDEMethod};
use crate::params::common::CommonParams;
use crate::traits::payoff::Payoff;
use crate::utils::linear_algebra::ThomasSolver;

#[derive(Debug,Clone)]
pub struct CrankNicolsonMethod;

impl CrankNicolsonMethod {
    pub fn new() -> CrankNicolsonMethod {
        Self
    }
}

impl PDEMethod for CrankNicolsonMethod {
    fn step_back(
        &self,
        grid: &mut Vec<Vec<f64>>,
        time_idx: usize,
        s_min: f64,
        dx: f64,
        dt: f64,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule,
        current_t: f64,
        use_log_space: bool
    ) -> Result<()> {
        let (s0,r,sigma,q,t_total)=params.all_params();
        let remaining_time=t_total-current_t;

        let to_price:fn(f64)->f64 = if use_log_space {|s| s.exp()}else{|s| s};

        let n=grid[time_idx].len();

        let mut a=vec![0.0; n];
        let mut b=vec![0.0; n];
        let mut c=vec![0.0; n];
        let mut rhs=vec![0.0; n];

        grid[time_idx][0]=grid[time_idx+1][0];
        grid[time_idx][n-1]=grid[time_idx+1][n-1];

        for i in 1..n-1{
            let s_space=s_min+i as f64*dx;
            let s=to_price(s_space);

            let alpha=if use_log_space{
                let log_diffusion=0.5*sigma.powi(2);
                log_diffusion*dt/(dx*dx)
            }else{
                0.5*sigma.powi(2)*s.powi(2)*dt/(dx*dx)
            };

            let beta = if use_log_space{
                let log_drift=r-q+0.5*sigma.powi(2);
                log_drift*dt/(2.0*dx)
            }else{
                (r-q)*s*dt/(2.0*dx)
            };

            a[i]=-0.5*alpha+0.5*beta;  // 下对角线
            b[i]=1.0+alpha+0.5*r*dt;   // 主对角线
            c[i]=-0.5*alpha-0.5*beta;  // 上对角线

            rhs[i]=-a[i]*grid[time_idx+1][i-1]
            +(2.0-b[i])*grid[time_idx+1][i]
            -c[i]*grid[time_idx+1][i+1];
        }
        // 边界条件处理
        let alpha_0=if use_log_space{
            0.5*sigma.powi(2)*dt/(dx*dx)
        }else{
            0.5*sigma.powi(2)*(0.1*s0).powi(2)*dt/(dx*dx)
        };

        let alpha_n=if use_log_space{
            0.5*sigma.powi(2)*dt/(dx*dx)
        }else{
            0.5*sigma.powi(2)*(2.0*s0).powi(2)*dt/(dx*dx)
        };

        // 边界条件调整
        rhs[0]=(1.0-alpha_0-0.5*r*dt)*grid[time_idx+1][0];
        rhs[n-1]=(1.0-alpha_n-0.5*r*dt)*grid[time_idx+1][n-1];
        b[0]=1.0+alpha_0+0.5*r*dt;
        b[n-1]=1.0+alpha_n+0.5*r*dt;

        rhs=ThomasSolver(&a,&b,&c,&rhs)?;

        for i in 1..n-1{
            let s_space=s_min+i as f64*dx;
            let s=to_price(s_space);
            let intrinsic_value=payoff.payoff(s);

            grid[time_idx][i]=if exercise_rule.should_exercise(remaining_time,s,intrinsic_value,rhs[i]){
                intrinsic_value
            }else{
                rhs[i]
            };
        }
        Ok(())
    }
}