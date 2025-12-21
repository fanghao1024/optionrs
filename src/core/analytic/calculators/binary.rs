use crate::errors::*;
use crate::params::common::CommonParams;
use crate::traits::engine::AnalyticCalculator;
use crate::traits::payoff::{AnalyticPayoffType, Payoff,CashOrNothingCallPayoff};
use crate::utils::statistics::{calculate_d1_d2, norm_cdf};

/// 二元期权
#[derive(Debug,Clone)]
pub struct BinaryCalculator;

impl AnalyticCalculator for BinaryCalculator {
    fn supported_types(&self) -> Vec<AnalyticPayoffType> {
        vec![AnalyticPayoffType::CashOrNothingCall,AnalyticPayoffType::CashOrNothingPut]
    }

    fn calculate(&self, params: &CommonParams, payoff: &dyn Payoff) -> Result<f64> {
        let s=params.spot();
        let r=params.risk_free_rate();
        let q=params.dividend_yield();
        let sigma=params.volatility();
        let t=params.time_to_maturity();

        if t==0.0{
            return Ok(payoff.payoff(s));
        }

        let (strike,payout,_is_call)=match payoff.as_any().downcast_ref::<CashOrNothingCallPayoff>(){
            Some(binary_call)=>{
                if binary_call.payout<0.0{
                    return Err(OptionError::InvalidParameter("The payout of binary call option must be greater than 0".to_string()));
                }
                (binary_call.strike,binary_call.payout,true)
            },
            // 此处后续扩展CashOrNothingPutPayoff、AssetOrNothingCallPayoff、AssetOrNothingPutPayoff
            None=>{
                return Err(OptionError::NotImplemented("Now only support cash-or-nothing call option.".to_string()));
            }
        };
        let (_,d2)=calculate_d1_d2(s,strike,r,q,sigma,t)?;
        let exp_rt=(-r*t).exp();

        let price=payout*exp_rt*norm_cdf(d2);
        Ok(price.max(0.0))
    }
}
