use std::any::Any;

/// 解析解期权类型枚举（标识不同期权类型)
#[derive(Debug,Clone,Copy,PartialEq,Hash,Eq)]
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


/// Define the interface for calculating option returns
///
/// 定义期权收益计算接口
pub trait Payoff:Send+Sync{
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

    /// 向下转型为Any（用于类型识别）
    fn as_any(&self)->&dyn Any;

    /// Obtain the type of parsing solution
    /// (return **None** if there is no parsing solution) <br>
    /// 获取解析解类型（无解析解则返回None)
    fn analytic_type(&self)->Option<AnalyticPayoffType>{
        None
    }
}

/// Vanilla call option<br>
/// 普通看涨期权
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
    fn analytic_type(&self)->Option<AnalyticPayoffType>{
        Some(AnalyticPayoffType::VanillaCall)
    }
}

/// Vanilla put option <br>
/// 普通看跌期权
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
    fn analytic_type(&self)->Option<AnalyticPayoffType>{
        Some(AnalyticPayoffType::VanillaPut)
    }
}

/// Cash or nothing call option payoff <br>
/// 现金或无看涨二元期权Payoff
#[derive(Debug,Clone,Copy)]
pub struct CashOrNothingCallPayoff{
    pub strike:f64,
    pub payout:f64, // 赔付额（二元期权专属）
}

impl Payoff for CashOrNothingCallPayoff{
    fn payoff(&self,spot:f64)->f64{
        if spot>=self.strike{self.payout} else {0.0}
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn analytic_type(&self)->Option<AnalyticPayoffType>{
        Some(AnalyticPayoffType::CashOrNothingCall)
    }
}

/// 现金或无看跌期权

/// Knock down the call barrier option payoff
/// 向下敲出看涨障碍期权Payoff
#[derive(Debug,Clone,Copy)]
pub struct DownAndOutCallPayoff{
    pub strike:f64,
    pub barrier:f64,
}

impl Payoff for DownAndOutCallPayoff{
    fn payoff(&self,spot:f64)->f64{
        todo!()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn analytic_type(&self)->Option<AnalyticPayoffType>{
        Some(AnalyticPayoffType::DownAndOutCall)
    }
}