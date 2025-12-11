//! 同样定价公式，包括margrabe，远期、期货和交换期权
use super::*;

/// 通用期权定价公式（基础公式）
pub fn generic_option(P1:f64,P2:f64,sigma:f64,T:f64)->f64{
    //输入参数
    //P1=将要得到的资产的现值
    //P2=将要支付的资产的现值
    //sigma=波动率
    //T=到期时间
    let x=((P1/P2).ln()+0.5*sigma.powi(2)*T)/(sigma*T.sqrt());
    let y=x-sigma*T.sqrt();
    let standard_norm=Normal::new(0.0,1.0).unwrap();
    //调用 standard_normal.cdf(x) 即可计算出 P (X ≤ x) 的值
    let n_d1=standard_norm.cdf(x);
    let n_d2=standard_norm.cdf(y);
    P1*n_d1-P2*n_d2
}

/// margrabe期权定价（资产交换期权）
pub fn margrabe(S1:f64,S2:f64,sigma:f64,q1:f64,q2:f64,T:f64)->f64{
    //输入参数
    //S1=将要得到的资产的价格
    //S2=将要支付的资产的价格
    //sigma=波动率
    //q1=将要得到的资产的红利支付率
    //q2=将要支付的资产的红利支付率
    //T=到期时间
    generic_option(S1*(-q1*T).exp(),S2*(-q2*T).exp(),sigma,T)
}

/// Black模型看涨期权定价（基于远期价格）
pub fn black_call(F:f64,K:f64,P:f64,sigma:f64,T:f64)->f64{
    //输入参数
    //F1=远期价格
    //K=执行价格
    //P=和远期合约同时到期的贴现债券价格
    //sigma=远期价格波动率
    //T=到期时间
    generic_option(F*P,K*P,sigma,T)

}

/// Black模型看跌期权定价
pub fn black_Put(F:f64,K:f64,P:f64,sigma:f64,T:f64)->f64{
    //输入参数
    //F1=远期价格
    //K=执行价格
    //P=和远期合约同时到期的贴现债券价格
    //simga=远期价格波动率
    //T=到期时间
    generic_option(K*P,F*P,sigma,T)
}

/// 延迟生效margrabe期权
pub fn margrabe_deferred(S1:f64,S2:f64,sigma:f64,q1:f64,q2:f64,Tm:f64,Te:f64)->f64{
    //输入参数
    //S1=将要得到的资产的价格
    //S2=将要支付的资产的价格
    //sigma=波动率
    //q1=将要得到的资产的红利支付率
    //q2=将要支付的资产的红利支付率
    //Tm=期权到期时间
    //Te=到交换发生的时间（大于等于Tm)
    generic_option(S1*(-q1*Te).exp(),S2*(-q2*Te).exp(),sigma,Tm)
}

/// Black模型看涨期权（重载实现）
pub fn black_call_2(F:f64,K:f64,P:f64,sigma:f64,T:f64)->f64{
    //
    // Inputs are F = forward price
    //            K = strike price
    //            P = price of discount bond maturing when forward matures
    //            sigma = volatility of forward price
    //            T = time to maturity
    //
    // To value a futures option, input F = futures price and P = price
    // of discount bond maturing when option matures.
    //

    if sigma==0.0{
        (P*(F-K)).max(0.0)
    }else{
        generic_option(P*F,P*K,sigma,T)
    }
}

/// Black模型看涨期权隐含波动率
pub fn black_call_implied_vol(F:f64,K:f64,P:f64,T:f64,CallPrice:f64)->Option<f64>{
    //
    //Inputs are F = initial forward price
    //           K = strike price
    //           P = price of zero-coupon bond maturing when forward matures (at T')
    //           T = time to maturity of call
    //           CallPrice = call price
    //
    let arbitrage_lower_bound=0.0_f64.max(P*(F-K));
    if CallPrice<arbitrage_lower_bound{
        eprintln!("期权价格违反无套利边界条件");
        return None
    }
    let tol=1e-6;
    let mut lower=0.0;
    let mut flower=black_call_2(F,K,P,lower,T)-CallPrice;
    let mut upper=1.0;
    let mut fupper=black_call_2(F,K,P,upper,T)-CallPrice;
    while fupper<0.0{
        upper*=2.0;
        fupper=black_call_2(F,K,P,upper,T)-CallPrice;
    }
    let mut guess=0.5*lower+0.5*upper;
    let mut fguess=black_call_2(F,K,P,guess,T)-CallPrice;

    while upper-lower>tol{
        if fupper*fguess<0.0{
            lower=guess;
            flower=fguess;
        }else{
            upper=guess;
            fupper=fguess;
        }
        guess=0.5*lower+0.5*upper;
        fguess=black_call_2(F,K,P,guess,T)-CallPrice;
    }
    Some(guess)

}

/// Black模型看涨期权Delta
pub fn black_call_delta(F:f64,K:f64,P:f64,sigma:f64,T:f64)->f64{
    /// 计算Black模型下远期/期货看涨期权的Delta值
    /// 参数：
    /// - F: 远期价格（期货价格）
    /// - K: 行权价格
    /// - P: 到期贴现债券价格
    /// - sigma: 远期价格波动率
    /// - T: 期权到期时间（年）
    /// 返回：Delta值（对冲头寸比例）
    // 到期时间为0时，Delta为0或1（内在价值区间）
    if T<=0.0{
        return if F>K {1.0*P}else{0.0*P};
    }
    // 波动率为0时，Delta同样是0/1（无时间价值）
    if sigma==0.0{
        return if F>K {1.0*P}else{0.0*P};
    }
    let d1=((F/K).ln()+0.5*sigma*sigma*T)/(sigma*T.sqrt());
    let standard_norm=Normal::new(0.0,1.0).unwrap();
    let N1=standard_norm.cdf(d1);
    P*N1
}