use std::any::Any;

/// 解析解期权类型枚举（标识不同期权类型)
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum AnalyticPayoffType{
    // 基础类型
    VanillaCall,
    VanillaPut,

    // 二元期权
    CashOrNothingCall,
    CashOrNothingPut,
    AssertOrNothingCall,
    AssertOrNothingPut,

    // barrier option 障碍期权
    DownAndOutCall,
    UpAndOutCall,

}

/// 解析解期权所需额外参数
#[derive(Debug,Clone,Copy)]
pub struct AnalyticPayoffParams{
    strike: f64,                // 基础行权价
    barrier: Option<f64>,       // 障碍价(障碍期权专用)
    payout: f64,       // 赔付额(二元期权专用)
    avg_period: Option<f64>,    // 平均周期
}

/// Define the interface for calculating option returns
///
/// 定义期权收益计算接口
pub trait Payoff{
    /// calculate option returns at a given assert price <br>
    /// 计算给定的资产价格下的期权收益
    /// ### parameter
    /// - spot: underlying assert price 标的资产价格
    fn payoff(&self,spot:f64)->f64;

    /// Calculate the returns of path dependent options
    /// (implemented as non path dependent by default) <br>
    /// 计算路径依赖期权的收益（默认实现为非路径依赖）
    /// - path: underlying path 标的资产路径
    fn path_dependent_payoff(&self,path:&[f64])->f64{
        /// Calculate profits using the last price by default
        /// 默认使用最后一个价格计算收益
        self.payoff(path.last().copied().unwrap_or(0.0))
    }

    fn as_any(&self)->&dyn Any;
}

/// Call option<br>
/// 看涨期权
#[derive(Debug,Clone,Copy)]
pub struct CallPayoff{
    pub strike:f64,
}

impl Payoff for CallPayoff{
    fn payoff(&self,spot:f64)->f64{
        (spot-self.strike).max(0.0)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Put option <br>
/// 看跌期权
#[derive(Debug,Clone,Copy)]
pub struct PutPayoff{
    pub strike:f64,
}

impl Payoff for PutPayoff{
    fn payoff(&self,spot:f64)->f64{
        (self.strike-spot).max(0.0)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
