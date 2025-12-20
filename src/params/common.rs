//! Common parameters for all type of options 所有期权通用的参数
use crate::errors::*;
#[derive(Debug,Clone,Copy)]
pub struct CommonParams{
    spot:f64,
    risk_free_rate:f64,
    volatility:f64,
    dividend_yield:f64,
    time_to_maturity:f64,
}

impl CommonParams{
    /// Create a new instance of universal parameters,including parameter validation
    /// 创建新的通用参数实例，包含参数验证
    pub fn new(
        spot:f64,
        risk_free_rate:f64,
        volatility:f64,
        dividend_yield:f64,
        time_to_maturity:f64,
    )->Result<Self>{
        let params=Self{
            spot,
            risk_free_rate,
            volatility,
            dividend_yield,
            time_to_maturity,
        };
        crate::utils::statistics::validate_common_params(&params)?;
        Ok(params)
    }

    // Getter method
    pub fn spot(&self) -> f64{self.spot}
    pub fn risk_free_rate(&self) -> f64{self.risk_free_rate}
    pub fn volatility(&self) -> f64{self.volatility}
    pub fn dividend_yield(&self) -> f64{self.dividend_yield}
    pub fn time_to_maturity(&self) -> f64{self.time_to_maturity}
    /// spot, risk_free_rate, volatility, dividend_yield, time_to_maturity
    pub fn all_params(&self)->(f64,f64,f64,f64,f64){
        (self.spot,self.risk_free_rate,self.volatility,self.dividend_yield,self.time_to_maturity)
    }

    /// Create a parameter copy of minor pertubations(for calculating Greek letters)<br>
    /// 创建微小扰动的参数副本（用于计算希腊字母）
    pub fn with_spot(&self, new_spot:f64)->Result<Self>{
        Self::new(
            new_spot,
            self.risk_free_rate,
            self.volatility,
            self.dividend_yield,
            self.time_to_maturity,
        )
    }

    /// Create a parameter copy of minor pertubations(for calculating Greek letters)<br>
    /// 创建微小扰动的参数副本（用于计算希腊字母）
    pub fn with_volatility(&self, new_volatility:f64)->Result<Self>{
        Self::new(
            self.spot,
            self.risk_free_rate,
            new_volatility,
            self.dividend_yield,
            self.time_to_maturity,
        )
    }

    /// Create a parameter copy of minor pertubations(for calculating Greek letters)<br>
    /// 创建微小扰动的参数副本（用于计算希腊字母）
    pub fn with_time(&self, new_maturity:f64)->Result<Self>{
        Self::new(
            self.spot,
            self.risk_free_rate,
            self.volatility,
            self.dividend_yield,
            new_maturity,
        )
    }

}