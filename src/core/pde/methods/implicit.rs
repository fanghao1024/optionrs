use crate::traits::engine::PDEMethod;
use crate::errors::*;
use crate::params::common::CommonParams;
use crate::traits::{payoff::Payoff,exercise::ExerciseRule};
use crate::utils::linear_algebra::ThomasSolver;

#[derive(Debug,Clone)]
pub struct ImplicitMethod;

impl ImplicitMethod {
    pub fn new()->Self{
        Self
    }
}


impl PDEMethod for ImplicitMethod {
    fn step_back(
        &self,
        grid: &mut Vec<Vec<f64>>,
        time_idx: usize,
        s_min: f64,                 // 空间网格的起始值（S轴最小值）
        dx: f64,                    // 空间网格步长（ΔS或Δx，取决于是否用对数空间
        dt: f64,                    // 时间步长（Δt）
        params: &CommonParams,      // Black-Scholes参数（r, σ, q, T等）
        payoff: &dyn Payoff,        // 期权收益函数（计算内在价值）
        exercise_rule: &dyn ExerciseRule,  // 行权规则（美式/欧式）
        current_t: f64,             // 当前时间t
        use_log_space: bool         // 是否用对数空间S=e^x（避免S=0的数值问题）
    ) -> Result<()> {
        let (s0,r,sigma,q,t_total)=params.all_params();
        let remaining_time=t_total-current_t;   // 剩余到期时间
        // 空间网格值转实际标的价格：对数空间则exp(x)，否则直接用x
        let to_price:fn(f64)->f64=if use_log_space { |s| s.exp() }else{|s| s};
        // 空间网格节点数N_s
        let n=grid[time_idx].len();

        // 构造三对角矩阵的系数
        let mut a=vec![0.0; n-1];
        let mut b=vec![0.0; n];
        let mut c=vec![0.0; n-1];
        let mut rhs=vec![0.0; n];

        b[0]=1.0;
        if n>1{c[0]=0.0;}
        rhs[0]=grid[time_idx][0];


        //填充矩阵系数
        for i in 1..n-1{
            let s_space=s_min+i as f64*dx;
            let s=to_price(s_space);
            // 二阶空间导数的系数α：对应(1/2)σ²S²/ΔS² * Δt（原始空间）或0.5σ²/Δx² * Δt（对数空间）
            let alpha=if use_log_space{
                0.5*sigma.powi(2)*dt/dx.powi(2)
            }else{
                0.5*sigma.powi(2)*s.powi(2)*dt/dx.powi(2)
            };

            let beta = if use_log_space{
                (r-q-0.5*sigma.powi(2))*dt/(2.0*dx)
            }else{
                (r-q)*s*dt/(2.0*dx)
            };

            a[i-1]=-alpha+beta;
            b[i]=1.0+2.0*alpha+r*dt;
            c[i]=-alpha-beta;

            rhs[i]=grid[time_idx+1][i];
        }

        b[n-1]=1.0;
        if n>2{
            a[n-2]=0.0;
        }
        rhs[n-1]=grid[time_idx][n-1];

        rhs=ThomasSolver(&a,&b,&c,&rhs)?;

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

