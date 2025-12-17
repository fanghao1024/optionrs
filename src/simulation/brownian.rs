//! 实现简单布朗运动（SBM）和几何布朗运动（GBM）
//! - 简单布朗运动：适用于无漂移/恒定波动率的随机位移（价格可能为负）
//! - 几何布朗运动：适用于股票价格等非负资产（核心公式：dS = μSdt + σSdW）

use rand::{Rng, SeedableRng,rngs::StdRng};
use rand_distr::StandardNormal;
use crate::traits::process::StochasticProcess;
use crate::errors::*;


/// 简单布朗运动（Standard Brownian Motion, SBM）
/// 核心公式：dW = ε·√dt，其中 ε ~ N(0,1)
/// 特性：
/// - 增量独立同分布
/// - 连续路径，处处不可微
/// - 位移对称，价格可能为负（不适合股票，适合利率/汇率）
#[derive(Debug,Clone)]
pub struct SimpleBrownianMotion{
    drift: f64,
    volatility: f64,
    rng:StdRng, // 随机数生成器
}

impl SimpleBrownianMotion{
    /// create standard simple brownian motion （μ=0, σ=1） <br>
    /// 创建标准的简单布朗运动 （μ=0, σ=1）
    pub fn standard()->Result<Self>{
        Ok(Self{
            drift:0.0,
            volatility:1.0,
            rng:StdRng::from_os_rng(),
        })
    }

    /// Create a custom simple brownian motion <br>
    /// 创建自定义的简单布朗运动
    pub fn new(drift:f64, volatility:f64)->Result<Self>{
        if volatility<0.0{
            return Err(OptionError::InvalidParameter("Volatility must be 0 or positive".to_string()));
        }
        Ok(Self{
            drift,
            volatility,
            rng:StdRng::from_os_rng(),
        })
    }

    /// Reset random number generator(specify seed to ensure reproducibility)
    /// 重置随机数生成器（指定种子，保证可复现）
    pub fn reset_rng(&mut self,seed:u64){
        self.rng = StdRng::seed_from_u64(seed);
    }
}

impl StochasticProcess for SimpleBrownianMotion{
    fn init_rng(&mut self, mut rng: impl Rng)
    where
        Self: Sized,
    {
        let seed:[u8;32]=rng.random();
        self.rng = StdRng::from_seed(seed);
    }

    fn next_step(&mut self,current_price:f64,time_step:f64)->Result<f64>{
        if time_step<=0.0{
            return Ok(current_price);
        }
        let epsilon:f64=self.rng.sample(StandardNormal);
        let drift_term=self.drift*time_step;
        let diffusion_term=self.volatility*epsilon*time_step.sqrt();
        Ok(current_price+drift_term*diffusion_term)
    }

    fn simulate_path(
        &mut self,
        initial_price:f64,
        time_horizon:f64,
        steps:usize
    )->Result<Vec<f64>>{
        if steps==0 || time_horizon<0.0{
            return Ok(vec![initial_price]);
        }
        let mut path=Vec::with_capacity(steps+1);
        path.push(initial_price);
        let dt=time_horizon/steps as f64;
        let drift_term=self.drift*dt;
        let diffusion_term=self.volatility*dt.sqrt();
        let mut current_price=initial_price;

        for i in 1..=steps{
            let epsilon:f64=self.rng.sample(StandardNormal);
            current_price+=drift_term+diffusion_term*epsilon;
            path.push(current_price);
        }
        Ok(path)
    }
}

/// 几何布朗运动（Geometric Brownian Motion, GBM）
/// 核心公式：dS = μSdt + σSdW → 离散形式：S(t+dt) = S(t)·exp[(μ-0.5σ²)dt + σ·ε·√dt]
/// 特性：
/// - 价格非负（适合股票、指数等资产）
/// - 对数正态分布（收益率正态分布）
/// - 无套利、马尔可夫性
#[derive(Debug,Clone)]
pub struct GeometricBrownianMotion{
    drift:f64,
    volatility: f64,
    rng:StdRng,
}
impl GeometricBrownianMotion{
    pub fn new(drift:f64, volatility:f64)->Result<Self>{
        if volatility<0.0{
            return Err(OptionError::InvalidParameter("Volatility must be 0 or positive".to_string()));
        }
        Ok(Self{
            drift,
            volatility,
            rng:StdRng::from_os_rng(),
        })
    }

    /// Convert from financial parameters <br>
    /// 直接由金融参数生成GBM实例
    /// ### parameter
    /// - risk_free_rate: 无风险利率(year)
    /// - dividend_yield: 连续支付红利率(year)
    /// - volatility: 波动率 σ
    pub fn from_financial_params(
        risk_free_rate:f64,
        dividend_yield:f64,
        volatility:f64,
    )->Result<Self>{
        let drift=risk_free_rate-dividend_yield;
        Self::new(drift, volatility)
    }

    /// Reset random number generator(specify seed to ensure reproducibility)
    /// 重置随机数生成器（指定种子，保证可复现）
    pub fn reset_rng(&mut self,seed:u64){
        self.rng = StdRng::seed_from_u64(seed);
    }
}

impl StochasticProcess for GeometricBrownianMotion{
    fn init_rng(&mut self, mut rng: impl Rng)
    where
        Self: Sized,
    {
        let seed:[u8;32]=rng.random();
        self.rng = StdRng::from_seed(seed);
    }
    fn next_step(&mut self,current_price:f64,time_step:f64)->Result<f64>{
        if time_step<=0.0 || current_price<0.0{
            return Ok(current_price);
        }
        let epsilon:f64=self.rng.sample(StandardNormal);
        let dt=time_step;
        let drift_term=(self.drift-0.5*self.volatility.powi(2))*dt;
        let diffusion_term=self.volatility*dt.sqrt()*epsilon;

        Ok(current_price*f64::exp(drift_term+diffusion_term))
    }
    fn simulate_path(
        &mut self,
        initial_price: f64,
        time_horizon: f64,
        steps: usize
    ) -> Result<Vec<f64>> {
        if initial_price<=0.0{
            return Err(OptionError::InvalidParameter("Initial price must be 0 or positive".to_string()));
        }
        if time_horizon<0.0{
            return Err(OptionError::InvalidParameter("Time horizon must be 0 or positive".to_string()));
        }

        let mut path=Vec::with_capacity(steps+1);
        path.push(initial_price);
        let dt=time_horizon/steps as f64;
        let drift_term=self.drift*dt;
        let diffusion_term=self.volatility*dt.sqrt();
        let mut log_s=initial_price.ln();

        for _ in 1..=steps{
            let epsilon:f64=self.rng.sample(StandardNormal);
            log_s+=drift_term+diffusion_term*epsilon;
            path.push(log_s.exp());
        }
        Ok(path)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    /// test the property of simple brownian motion
    #[test]
    fn test_simple_brownian_motion()->Result<()>{
        let mut sbm=SimpleBrownianMotion::standard()?;
        sbm.reset_rng(43);

        let path=sbm.simulate_path(0.0,1.0,252)?;
        assert_eq!(path.len(),253);
        assert_eq!(path[0],0.0);

        for i in 1..path.len(){
            let step=(path[i]-path[i-1]).abs();
            assert!(step<10.0);
        }

        Ok(())
    }

    /// test the non negativity of Geometric Brownian Motion
    #[test]
    fn test_geometric_brownian_motion()->Result<()>{
        let mut gbm=GeometricBrownianMotion::from_financial_params(0.05,0.02,0.2)?;
        gbm.reset_rng(43);

        let path=gbm.simulate_path(100.0,1.0,252)?;
        assert_eq!(path.len(),253);
        assert!(path.iter().all(|&x|x>0.0));
        Ok(())
    }

    /// Test the expected return rate of GBM(validation of the law of large numbers)
    #[test]
    fn test_gbm_expected_return()->Result<()>{
        let mut gbm=GeometricBrownianMotion::new(0.05,0.2)?;
        gbm.reset_rng(43);

        let mut final_price=Vec::with_capacity(100000);
        for _ in 0..1000{
            let path=gbm.simulate_path(100.0,1.0,252)?;
            final_price.push(*path.last().unwrap());
        }

        let avg_final=final_price.iter().sum::<f64>() / final_price.len() as f64;
        let avg_return = (avg_final/100.0).ln();
        println!("avg final={}", avg_return);
        assert!((avg_return-0.05).abs() < 0.03);
        Ok(())
    }
}
