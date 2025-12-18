use std::any::Any;
use crate::errors::*;
use crate::traits::engine::{PriceEngine,GreeksEngine};
use crate::params::common::CommonParams;
use crate::traits::{payoff::Payoff, exercise::ExerciseRule};
use crate::traits::exercise::EuropeanExercise;
use crate::utils::distributions::{norm_cdf,norm_pdf};

#[derive(Debug,Clone,Copy)]
pub struct AnalyticEngine;

impl AnalyticEngine {
    fn calculate_d1_d2(&self,params:&CommonParams,strike:f64)->Result<(f64,f64)>{
        let s=params.spot();
        let r=params.risk_free_rate();
        let q=params.dividend_yield();
        let sigma=params.volatility();
        let t=params.time_to_maturity();
        let k=strike;

        let d1=(s.ln()-k.ln()+(r-q+0.5*sigma.powi(2))*t)/(sigma*t.sqrt());
        let d2=d1-sigma*t.sqrt();
        Ok((d1,d2))
    }

}

impl PriceEngine for AnalyticEngine {
    fn price(
        &self,
        params: &CommonParams,
        payoff: &dyn Payoff,
        exercise_rule: &dyn ExerciseRule
    ) -> Result<f64> {
        if exercise_rule.as_any().downcast_ref::<EuropeanExercise>().is_none(){
            return Err(OptionError::InvalidParameter("Now AnalyticEngine can only support European exercise rule.".to_string()));
        }
        
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

