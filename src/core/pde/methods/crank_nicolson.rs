use crate::errors::*;
use crate::traits::exercise::ExerciseRule;
use crate::traits::engine::{PDEMethod};
use crate::params::common::CommonParams;
use crate::traits::payoff::Payoff;
use crate::utils::linear_algebra::thomas_solver;

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
        let (_,r,sigma,q,t_total)=params.all_params();
        let remaining_time=t_total-current_t;

        let to_price:fn(f64)->f64 = if use_log_space {|s| s.exp()}else{|s| s};

        let n=grid[time_idx].len();

        let mut a=vec![0.0; n-1];
        let mut b=vec![0.0; n];
        let mut c=vec![0.0; n-1];
        let mut rhs=vec![0.0; n];

        b[0]=1.0;
        if n>1{c[0]=0.0;}
        rhs[0]=grid[time_idx][0];


        for i in 1..n-1{
            let s_space=s_min+i as f64*dx;
            let s=to_price(s_space);

            let alpha=if use_log_space{
                0.5*sigma.powi(2)*dt/(dx*dx)
            }else{
                0.5*sigma.powi(2)*s.powi(2)*dt/(dx*dx)
            };

            let beta = if use_log_space{
                (r-q-0.5*sigma.powi(2))*dt/(2.0*dx)
            }else{
                (r-q)*s*dt/(2.0*dx)
            };

            a[i]=-0.5*alpha+0.5*beta;  // 下对角线
            b[i]=1.0+alpha+0.5*r*dt;   // 主对角线
            c[i]=-0.5*alpha-0.5*beta;  // 上对角线

            rhs[i]=-a[i]*grid[time_idx+1][i-1]
            +(1.0-alpha-0.5*r*dt)*grid[time_idx+1][i]
            -c[i]*grid[time_idx+1][i+1];
        }

        b[n-1]=1.0;
        if n>1{a[n-2]=0.0;}
        rhs[n-1]=grid[time_idx][n-1];

        rhs=thomas_solver(&a,&b,&c,&rhs)?;

        for i in 0..n{
            let s_space=s_min+i as f64*dx;
            let s=to_price(s_space);
            let intrinsic_value=payoff.payoff(s);

            if i>0 && i<n-1{
                grid[time_idx][i]=if exercise_rule.should_exercise(remaining_time,s,intrinsic_value,rhs[i]){
                    intrinsic_value
                }else{
                    rhs[i]
                };
            }else{
                grid[time_idx][i]=rhs[i];
            }
        }
        Ok(())
    }
}

