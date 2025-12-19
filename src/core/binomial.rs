use std::any::Any;
use crate::errors::*;
use crate::traits::engine::{PriceEngine,GreeksEngine,BinomialEngineExt};
use crate::params::common::CommonParams;
use crate::traits::{payoff::Payoff, exercise::ExerciseRule};

#[derive(Debug,Clone)]
pub struct BinomialEngine{
    steps:usize,
}

impl BinomialEngine {
    pub fn new(steps:usize)->Result<Self>{
        if steps<10{
            return Err(OptionError::InvalidParameter("The steps of binomial Tree cannot be less than 10 steps.".into()));
        }
        Ok(Self{steps})
    }
    pub fn with_steps(steps:usize)->Result<Self>{
        Self::new(steps)
    }
}

impl PriceEngine for BinomialEngine {
    fn price(
        &self,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule
    ) -> Result<f64> {
        let s=params.spot();
        let r=params.risk_free_rate();
        let q=params.dividend_yield();
        let sigma=params.volatility();
        let t=params.time_to_maturity();

        if t<=0.0{
            return Ok(payoff.payoff(s));
        }

        let dt=t/self.steps as f64;
        let u=(sigma*dt.sqrt()).exp();
        let d=1.0/u;
        let a=(r-q)*dt;
        let disc=(-r*dt).exp();
        let p=(a.exp()-d)/(u-d);
        let p_u=p*disc;
        let p_d=(1.0-p)*disc;

        let mut option_values=vec![0.0;self.steps+1];
        let mut s_current=s*d.powi(self.steps as i32);

        for i in 0..=self.steps{
            option_values[i]=payoff.payoff(s_current);
            s_current*=u*u;
        }

        for j in (0..self.steps).rev(){
            for i in 0..=j{
                let continuation_value=p_u*option_values[i+1]+p_d*option_values[i];
                let s_current=s*u.powi(2*i as i32-j as i32);
                let intrinsic_value=payoff.payoff(s_current);
                let remaining_time=t-j as f64*dt;

                option_values[i]=if exercise_rule.should_exercise(remaining_time,s_current,intrinsic_value,continuation_value){
                    intrinsic_value
                }else{
                    continuation_value
                };
            }
        }
        Ok(option_values[0])
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl BinomialEngineExt for BinomialEngine {
    fn set_steps(&mut self, steps: usize) -> Result<()> {
        if steps<10{
            return Err(OptionError::InvalidParameter("The steps of binomial Tree cannot be less than 10 steps.".into()));
        }
        self.steps = steps;
        Ok(())
    }
    fn get_steps(&self)->usize{
        self.steps
    }

}

impl GreeksEngine for BinomialEngine {}


unsafe impl Send for BinomialEngine {}
unsafe impl Sync for BinomialEngine {}