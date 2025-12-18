//! The specific implementation of Monte Carlo Engine
//! 蒙特卡洛引擎的具体实现
use rand::{Rng, SeedableRng,rngs::StdRng};
use std::any::Any;
use crate::traits::engine::{PriceEngine,GreeksEngine,MonteCarloEngineExt};
use crate::traits::{payoff::Payoff,exercise::ExerciseRule,process::StochasticProcess};
use crate::params::common::CommonParams;
use crate::simulation::brownian::GeometricBrownianMotion;
use crate::errors::*;
/// Monte Carlo Engine
#[derive(Debug,Clone)]
pub struct MonteCarloEngine{
    num_simulations: usize,        //模拟次数
    time_steps: usize,             //时间步数
    process: Option<Box<dyn StochasticProcess>>, //随机过程
    rng: StdRng,                   //随机数生成器
    use_antithetic:bool,           //是否启用对偶
}

impl MonteCarloEngine: {
    pub fn new(
        num_simulations: usize,
        time_steps: usize,
    ) -> Result<Self>{
        if num_simulations<1000{
            return Err(OptionError::InvalidParameter("Simulation number cannot be below 1000".to_string()));
        }
        if time_steps<1{
            return Err(OptionError::InvalidParameter("Time steps must be over 0".to_string()));
        }
        Ok(Self{
            num_simulations,
            time_steps,
            process:None,
            rng:StdRng::from_os_rng(),
            use_antithetic:false,
        })
    }

    pub fn set_antithetic(&mut self, use_antithetic:bool){
        self.use_antithetic = use_antithetic;
    }
}

impl MonteCarloEngineExt for MonteCarloEngine {
    fn set_process(&mut self, process: Box<dyn StochasticProcess>) {
        self.process=Some(process);
    }

    fn set_num_simulation(&mut self, num: usize) -> Result<()> {
        if num==0{
            return Err(OptionError::InvalidParameter("Simulation number must be greater than 0".to_string()));
        }
        self.num_simulations=num;
        Ok(())
    }

    fn set_time_steps(&mut self, time_steps: usize) -> Result<()> {
        if time_steps==0{
            return Err(OptionError::InvalidParameter("Time steps must be greater than 0".to_string()));
        }
        self.time_steps=time_steps;
        Ok(())
    }
}

impl PriceEngine for MonteCarloEngine {
    fn price(
        &self,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule
    ) -> Result<f64> {
        //let process=self.process.as_ref().ok_or(OptionError::NotSet("Specific stochastic process not set".to_string()))?;
        let mut process=match &self.process{
            Some(p) => p.clone_box(),
            None=>{
                let drift=params.risk_free_rate()-params.dividend_yield();
                let gbm=GeometricBrownianMotion::new(drift,params.volatility())?;
                Box::new(gbm)
            }
        };

        let s0=params.spot();
        let t=params.time_to_maturity();
        let r=params.risk_free_rate();

        let discount_factor=(-r*t).exp();
        let mut total_payoff=0.0;
        let mut rng=self.rng.clone();
        let effective_simulations=if self.use_antithetic{self.num_simulations/2}else{self.num_simulations};

        for _ in 0..effective_simulations {
            if self.use_antithetic {
                let (path1,path2)=process.simulate_antithetic_path(s0,t,self.time_steps)?;
                let payoff_value1=payoff.path_dependent_payoff(&path1);
                let payoff_value2=payoff.path_dependent_payoff(&path2);
                total_payoff+=payoff_value1;
                total_payoff+=payoff_value2;
            }else{
                let path=process.simulate_path(s0,t,self.time_steps)?;
                let payoff_value=payoff.path_dependent_payoff(&path);
                total_payoff+=payoff_value;
            }
        }
        let mean_payoff=total_payoff/self.num_simulations as f64;
        Ok(mean_payoff*discount_factor)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl GreeksEngine for MonteCarloEngine {}
