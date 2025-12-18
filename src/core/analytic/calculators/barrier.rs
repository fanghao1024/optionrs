use crate::errors::*;
use crate::params::common::CommonParams;
use crate::traits::engine::AnalyticCalculator;
use crate::traits::payoff::{AnalyticPayoffType, Payoff,DownAndOutCallPayoff};
use crate::traits::payoff::AnalyticPayoffType::DownAndOutCall;
use crate::utils::statistics::{calculate_d1_d2, norm_cdf};

#[derive(Debug,Clone)]
pub struct BarrierCalculator;

impl AnalyticCalculator for BarrierCalculator {
    fn supported_types(&self) -> Vec<AnalyticPayoffType> {
        vec![AnalyticPayoffType::DownAndOutCall,AnalyticPayoffType::UpAndOutCall]
    }

    fn calculate(&self, params: &CommonParams, payoff: &dyn Payoff) -> Result<f64> {
        let (s,r,q,sigma,t)=params.all_params();

        if t==0.0{
            return Ok(payoff.payoff(s));
        }

        let (strike,barrier,is_call)=match payoff.as_any().downcast_ref::<DownAndOutCallPayoff>(){
            Some(down_and_out_call)=>{
                if down_and_out_call.barrier<=0.0{
                    return Err(
                        OptionError::InvalidParameter(
                            "The barrier price for knocking down a call option \
                            must be negative".into()));
                }
                (down_and_out_call.strike,down_and_out_call.barrier,true)
            },
            None=>{
                return Err(OptionError::InvalidParameter("Now only support knock down call option".into()));
            }
        };
        let a;
        let b;
        if strike>barrier{
            a=s/strike;
            b=barrier*barrier/(strike*s);
        }else{
            a=s/barrier;
            b=barrier/s;
        }
        let d1=(a.ln()+(r-q+0.5*sigma*sigma)*t)/(sigma*t.sqrt());
        let d2=d1-sigma*t.sqrt();
        let d1prime=(b.ln()+(r-q+0.5*sigma*sigma)*t)/(sigma*t.sqrt());
        let d2prime=d1prime-sigma*t.sqrt();
        let N1=norm_cdf(d1);
        let N2=norm_cdf(d2);
        let N1prime=norm_cdf(d1prime);
        let N2prime=norm_cdf(d2prime);
        let x=1.0+2.0*(r-q)/(sigma*sigma);
        let y=x-2.0;
        let q1=N1-(barrier/s).powf(x)*N1prime;
        let q2=N2-(barrier/s).powf(y)*N2prime;
        Ok((-q*t).exp()*s*q1-(-r*t).exp()*strike*q2)
    }
}