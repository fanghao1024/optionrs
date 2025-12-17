//! Common parameters for all type of options 所有期权通用的参数
use crate::errors::*;
#[derive(Debug,Clone,Copy)]
pub struct CommonParams{
    spot:f64,
    risk_free_rate:f64,
    volatility:f64,
    dividened_yield:f64,
    time_to_maturity:f64,
}

impl CommonParams{
    /// Create a new instance of universal parameters,including parameter validation
    /// 创建新的通用参数实例，包含参数验证
    pub fn new(
        spot:f64,
        risk_free_rate:f64,
        volatility:f64,
        dividened_yield:f64,
        time_to_maturity:f64,
    )->Result<Self>{
        if spot<=0.0{
            return Err(OptionError::InvalidParameter("The price of underlying assert must be greater than 0".to_string()));
        }
        if volatility<0.0{
            return Err(OptionError::InvalidParameter("The volatility of underlying assert must be greater than 0".to_string()));
        }
        if time_to_maturity<0.0{
            return Err(OptionError::InvalidParameter("The time to maturity of underlying assert must be greater than 0".to_string()));
        }
        Ok(Self{
            spot,
            risk_free_rate,
            volatility,
            dividened_yield,
            time_to_maturity,
        })
    }

    // Getter method
    pub fn spot(&self) -> f64{self.spot}
    pub fn risk_free_rate(&self) -> f64{self.risk_free_rate}
    pub fn volatility(&self) -> f64{self.volatility}
    pub fn dividened_yield(&self) -> f64{self.dividened_yield}
    pub fn time_to_maturity(&self) -> f64{self.time_to_maturity}

    /// Create a parameter copy of minor pertubations(for calculating Greek letters)<br>
    /// 创建微小扰动的参数副本（用于计算希腊字母）
    pub fn with_spot(&self, new_spot:f64)->Result<Self>{
        Self::new(
            new_spot,
            self.risk_free_rate,
            self.volatility,
            self.dividened_yield,
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
            self.dividened_yield,
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
            self.dividened_yield,
            new_maturity,
        )
    }

}