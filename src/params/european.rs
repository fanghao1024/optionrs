use crate::params::common::CommonParams;
use crate::errors::*;
/// Unique parameters of European options <br>
/// 欧式期权特有参数
#[derive(Debug,Clone,Copy)]
pub struct EuropeanParams{
    common:CommonParams,
    strike:f64
}

impl EuropeanParams{
    pub fn new(
        common:CommonParams,
        strike:f64
    )->Result<Self>{
        if strike<0.0{
            return Err(OptionError::InvalidParameter("Strike price cannot be negative".to_string()));
        }
        Ok(Self{
            common,
            strike
        })
    }

    // Getter method
    pub fn common(&self)->&CommonParams{&self.common}
    pub fn strike(&self)->f64{self.strike}
}