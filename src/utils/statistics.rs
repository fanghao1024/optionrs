use statrs::distribution::{Normal,Continuous,ContinuousCDF};
use owens_t;
use crate::errors::*;

/// Standard normal distribution CDF <br>
/// 标准正态分布的CDF（累积分布函数）
pub fn norm_cdf(x:f64)->f64{
    let normal=Normal::new(0.0, 1.0).expect("Failed to create normal distribution");
    normal.cdf(x)
}

/// Standard normal distribution pdf <br>
/// 标准正态分布的PDF（概率密度函数)
pub fn norm_pdf(x:f64)->f64{
    let normal=Normal::new(0.0, 1.0).expect("Failed to create normal distribution");
    normal.pdf(x)
}

/// calculate the d1 and d2 of Black-Scholes formula
pub fn calculate_d1_d2(
    spot: f64,
    strike: f64,
    risk_free_rate: f64,
    dividend_yield:f64,
    volatility: f64,
    time_to_maturity: f64,
)->Result<(f64,f64)>{
    if spot<=0.0{
        return Err(OptionError::InvalidParameter("Spot must be greater than zero.".to_owned()).into());
    }
    if strike<=0.0{
        return Err(OptionError::InvalidParameter("Strike must be greater than zero.".to_owned()).into());
    }
    if volatility<=0.0{
        return Err(OptionError::InvalidParameter("Volatility must be greater than zero.".to_owned()).into());
    }
    if time_to_maturity<0.0{
        return Err(OptionError::InvalidParameter("Time to maturity cannot be negative.".to_owned()).into());
    }
    if time_to_maturity==0.0{
        return Err(OptionError::InvalidParameter("When the expiration time is 0,there is \
        no analytic solution (return intrinsic value directly)".to_owned()).into());
    }
    let ln_sk=(spot/strike).ln();
    let sigma_sqrt_t=volatility*time_to_maturity.sqrt();
    let d1=(ln_sk+(risk_free_rate-dividend_yield+0.5*volatility.powi(2)*time_to_maturity))/sigma_sqrt_t;
    let d2=d1-sigma_sqrt_t;
    Ok((d1,d2))
}

/// CDF of binary normal distribution <br>
/// 二元正态分布的CDF
pub fn bivariate_norm_cdf(a: f64, b: f64, rho: f64) -> f64 {
    use owens_t::biv_norm;

    let phi_a = norm_cdf(a);
    let phi_b = norm_cdf(b);
    let p_gt_a_gt_b = biv_norm(a, b, rho);

    // P(<a, <b) = P(<a) + P(<b) - 1 + P(>a, >b)
    phi_a + phi_b - 1.0 + p_gt_a_gt_b
}

/// 通用参数校验
pub fn validate_common_params(params:&crate::params::common::CommonParams)->Result<()>{
    if params.spot()<=0.0{
        return Err(OptionError::InvalidParameter("Spot must be greater than zero.".into()));
    }
    if params.volatility()<=0.0{
        return Err(OptionError::InvalidParameter("Volatility must be greater than zero.".into()));
    }
    if params.time_to_maturity()<0.0{
        return Err(OptionError::InvalidParameter("Time to maturity cannot be negative.".into()));
    }
    Ok(())
}
