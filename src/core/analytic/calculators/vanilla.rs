use rand_distr::num_traits::Float;
use crate::errors::OptionError;
use crate::traits::engine::AnalyticCalculator;
use crate::traits::payoff::{AnalyticPayoffType, CallPayoff, Payoff, PutPayoff};
use crate::params::common::CommonParams;
use crate::utils::statistics::{norm_cdf,calculate_d1_d2};

#[derive(Debug,Clone)]
pub struct VanillaCalculator;

impl AnalyticCalculator for VanillaCalculator{
    fn supported_types(&self) -> Vec<AnalyticPayoffType> {
        vec![AnalyticPayoffType::VanillaCall,AnalyticPayoffType::VanillaPut]
    }

    fn calculate(
        &self,
        params: &CommonParams,
        payoff: &dyn Payoff
    ) -> crate::errors::Result<f64> {
        let s=params.spot();
        let r=params.risk_free_rate();
        let q=params.dividend_yield();
        let sigma=params.volatility();
        let t=params.time_to_maturity();

        if t==0.0{
            return Ok(payoff.payoff(s));
        }

        let (strike,is_call)=match payoff.as_any().downcast_ref::<CallPayoff>(){
            Some(call)=>(call.strike,true),
            None=>match payoff.as_any().downcast_ref::<PutPayoff>(){
                Some(put)=>(put.strike,false),
                None=> return Err(OptionError::InvalidParameter("Vanilla calculator only support \
                vanilla call/put option".into())),
            }
        };
        let (d1,d2)=calculate_d1_d2(s,strike,r,q,sigma,t)?;
        let exp_qt=(-q*t).exp();
        let exp_rt=(-r*t).exp();

        let price=if is_call{
            s*exp_qt*norm_cdf(d1)-strike*exp_rt*norm_cdf(d2)
        }else{
            strike*exp_rt*norm_cdf(-d2)-s*exp_qt*norm_cdf(-d1)
        };
        Ok(price)
    }
}