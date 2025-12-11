//! 奇异期权定价模块

use super::*;
use crate::utils::bivariate_standard_normal_cdf;

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

/// 远期生效看涨期权
pub fn forward_start_call(S:f64,r:f64,sigma:f64,q:f64,Tset:f64,TCall:f64)->f64{
    let d_1=(r-q+0.5*sigma*sigma)*(TCall-Tset).sqrt()/sigma;
    let d_2=d_1-sigma*(TCall-Tset).sqrt();
    let normal=Normal::new(0.0, 1.0).unwrap();
    (-q*TCall).exp()*S*normal.cdf(d_1)-(-q*Tset-r*(TCall-Tset)).exp()*S*normal.cdf(d_2)

}


//下跌生效看涨期权
pub fn down_and_out_call(S:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64,Barrier:f64)->f64{
    ///输入参数
    /// S=初始股票价格
    /// K=执行价格
    /// r=无风险利率
    /// sigma=波动率
    /// q=红利支付率
    /// T=到期时间
    /// Barrier=敲出边界(<S)
    let a;
    let b;
    if K>Barrier{
        a=S/K;
        b=Barrier*Barrier/(K*S);
    }else{
        a=S/Barrier;
        b=Barrier/S;
    }
    let d1=(a.ln()+(r-q+0.5*sigma*sigma)*T)/(sigma*T.sqrt());
    let d2=d1-sigma*T.sqrt();
    let d1prime=(b.ln()+(r-q+0.5*sigma*sigma)*T)/(sigma*T.sqrt());
    let d2prime=d1prime-sigma*T.sqrt();
    let normal=Normal::new(0.0,1.0).unwrap();
    let N1=normal.cdf(d1);
    let N2=normal.cdf(d2);
    let N1prime=normal.cdf(d1prime);
    let N2prime=normal.cdf(d2prime);
    let x=1.0+2.0*(r-q)/(sigma*sigma);
    let y=x-2.0;
    let q1=N1-(Barrier/S).powf(x)*N1prime;
    let q2=N2-(Barrier/S).powf(y)*N2prime;
    (-q*T).exp()*S*q1-(-r*T).exp()*K*q2
}

//浮动执行价格回望看涨期权
pub fn floating_strike_call(S:f64,r:f64,sigma:f64,q:f64,T:f64,SMin:f64)->f64{
    ///输入参数
    /// S=初始价格
    /// r=无风险利率
    /// sigma=波动率
    /// q=红利支付率
    /// T=到期时间
    /// SMin=合约过去时间中的最低股票价格
    let d1=((S/SMin).ln()+(r-q+0.5*sigma*sigma)*T)/(sigma*T.sqrt());
    let d2=d1-sigma*T.sqrt();
    let d2prime=((SMin/S).ln()+(r-q-0.5*sigma*sigma)*T)/(sigma*T.sqrt());
    let normal=Normal::new(0.0,1.0).unwrap();
    let N1=normal.cdf(d1);
    let N2=normal.cdf(d2);
    let N2prime=normal.cdf(d2prime);
    let x=2.0*(r-q)/(sigma*sigma);
    (-q*T).exp()*S*N1-(-r*T).exp()*SMin*N2+(1.0/x)*(SMin/S).powf(x)*(-r*T).exp()*SMin*N2prime-(1.0/x)*(-q*T).exp()*S*(1.0-N1)
}

pub fn discrete_geom_average_price_call(S:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64,N:f64)->f64{
    ///输入参数
    /// S=初始股票价格
    /// K=执行价格
    /// r=无风险利率
    /// sigma=波动率
    /// q=红利支付率
    /// T=到期时间
    /// N=时间区间个数
    let dt=T/N;
    let nu=r-q-0.5*sigma*sigma;
    let a=N*(N+1.0)*(2.0*N+1.0)/6.0;
    let V=(-r*T).exp()*S*(((N+1.0)*nu/2.0+sigma*sigma*a/(2.0*N*N))*dt).exp();
    let sigavg=sigma*a.sqrt()/(N.powf(1.5));
    crate::black_scholes::european_call(V,K,r,sigavg,0.0,T)
}


pub fn find_sstar_call(Kc:f64,Ku:f64,r:f64,sigma:f64,q:f64,Tc:f64,Tu:f64)->f64{
    let tol=1e-6;
    let mut lower=0.0;
    let mut upper=(q*(Tu-Tc)).exp()*(Ku+Kc);
    let mut guess=0.5*upper+0.5*lower;
    let mut flower=-Kc;
    let mut fupper=crate::black_scholes::european_call(upper,Ku,r,sigma,q,Tu-Tc)-Kc;
    let mut fguess=crate::black_scholes::european_call(guess,Ku,r,sigma,q,Tu-Tc)-Kc;
    while upper-lower>tol{
        if fupper*fguess<0.0{
            lower=guess;
            flower=fguess;
            guess=0.5*upper+0.5*lower;
            fguess=crate::black_scholes::european_call(guess,Ku,r,sigma,q,Tu-Tc)-Kc;
        }else{
            upper=guess;
            fupper=fguess;
            guess=0.5*upper+0.5*lower;
            fguess=crate::black_scholes::european_call(guess,Ku,r,sigma,q,Tu-Tc)-Kc;
        }
    }
    guess

}

pub fn call_on_call(S:f64,Kc:f64,Ku:f64,r:f64,sigma:f64,q:f64,Tc:f64,Tu:f64)->f64{
    let Sstar=find_sstar_call(Kc,Ku,r,sigma,q,Tc,Tu);
    let d_1=((S/Sstar).ln()+(r-q+0.5*sigma*sigma)*Tc)/(sigma*Tc.sqrt());
    let d_2=d_1-(sigma*Tc.sqrt());
    let nd_1=((S/Ku).ln()+(r-q+0.5*sigma*sigma)*Tu)/(sigma*Tu.sqrt());
    let nd_2=nd_1-sigma*Tu.sqrt();
    let rho=(Tc/Tu).sqrt();
    let normal=Normal::new(0.0, 1.0).unwrap();
    -(-r*Tc).exp()*Kc*normal.cdf(d_2)+(-q*Tu).exp()*S*bivariate_standard_normal_cdf(d_1,nd_1,rho)-(-r*Tu).exp()*Ku*bivariate_standard_normal_cdf(d_2,nd_2,rho)
}

pub fn call_on_put(S:f64,Kc:f64,Ku:f64,r:f64,sigma:f64,q:f64,Tc:f64,Tu:f64)->f64{
    let tol=1e-6;
    let mut lower=0.0;
    let mut flower=(-r*(Tu-Tc)).exp()*Ku-Kc;
    let mut upper=2.0*Ku;
    let mut fupper=crate::black_scholes::european_put(upper,Ku,r,sigma,q,Tu-Tc)-Kc;
    while fupper>0.0{
        upper*=2.0;
        fupper=crate::black_scholes::european_put(upper,Ku,r,sigma,q,Tu-Tc)-Kc;
    }

    let mut guess=0.5*upper+0.5*lower;
    let mut fguess=crate::black_scholes::european_put(guess,Ku,r,sigma,q,Tu-Tc)-Kc;
    while upper-lower>tol{
        if fupper*fguess<0.0{
            lower=guess;
            flower=fguess;
            guess=0.5*upper+0.5*lower;
            fguess=crate::black_scholes::european_put(guess,Ku,r,sigma,q,Tu-Tc)-Kc;
        }else{
            upper=guess;
            fupper=fguess;
            guess=0.5*upper+0.5*lower;
            fguess=crate::black_scholes::european_put(guess,Ku,r,sigma,q,Tu-Tc)-Kc;
        }
    }
    let Sstar=guess;
    let normal=Normal::new(0.0, 1.0).unwrap();
    let d_1=((S/Sstar).ln()+(r-q+0.5*sigma*sigma)*Tc)/(sigma*Tc.sqrt());
    let d_2=d_1-(sigma*Tc.sqrt());
    let nd_1=((S/Ku).ln()+(r-q+0.5*sigma*sigma)*Tu)/(sigma*Tu.sqrt());
    let nd_2=nd_1-sigma*Tu.sqrt();
    let rho=(Tc/Tu).sqrt();

    -(-r*Tc).exp()*Kc*normal.cdf(-d_2)+(-r*Tu).exp()*Ku*bivariate_standard_normal_cdf(-d_2,-nd_2,rho)-(-q*Tu).exp()*S*bivariate_standard_normal_cdf(-d_1,-nd_1,rho)
}

pub fn american_call_dividened(S:f64,K:f64,r:f64,sigma:f64,Div:f64,TDiv:f64,TCall:f64)->f64{
    ///输入参数：
    /// S    = 初始股票价格
    /// r    = 无风险利率
    /// sigma= 波动率
    /// Div  = 现金红利
    /// TDiv = 距离红利支付的时间
    /// TCall= 期权到期时间(>=TDiv)
    let LessDiv=S-(-r*TDiv).exp()*Div;

    //绝对不会提早执行期权的情况
    if Div/K <= 1.0-(-r*(TCall-TDiv)).exp(){
        return crate::black_scholes::european_call(LessDiv,K,r,sigma,0.0,TCall);
    }

    let mut upper=K;
    while upper+Div-K<crate::black_scholes::european_call(upper,K,r,sigma,0.0,TCall-TDiv){
        upper*=2.0;
    }

    let tol=1e-6;
    let mut lower=0.0;
    let mut flower=Div-K;
    let mut fupper=upper+Div-K-crate::black_scholes::european_call(upper,K,r,sigma,0.0,TCall-TDiv);
    let mut guess=0.5*upper+0.5*lower;
    let mut fguess=guess+Div-K-crate::black_scholes::european_call(guess,K,r,sigma,0.0,TCall-TDiv);
    while upper-lower>tol{
        if fupper*fguess<0.0{
            lower=guess;
            flower=fguess;
            guess=0.5*upper+0.5*lower;
            fguess=guess+Div-K-crate::black_scholes::european_call(guess,K,r,sigma,0.0,TCall-TDiv);
        }else{
            upper=guess;
            fupper=fguess;
            guess=0.5*upper+0.5*lower;
            fguess=guess+Div-K-crate::black_scholes::european_call(guess,K,r,sigma,0.0,TCall-TDiv);
        }
    }

    let LessDivStar=guess;
    let d_1=((LessDiv/LessDivStar).ln()+(r+0.5*sigma*sigma)*TDiv)/(sigma*TDiv.sqrt());
    let d_2=d_1-sigma*TDiv.sqrt();
    let nd_1=((LessDiv/K).ln()+(r+0.5*sigma*sigma)*TCall)/(sigma*TCall.sqrt());
    let nd_2=nd_1-sigma*TCall.sqrt();
    let rho=-(TDiv/TCall).sqrt();
    let normal=Normal::new(0.0, 1.0).unwrap();
    let N1=normal.cdf(d_1);
    let N2=normal.cdf(d_2);
    let M1=bivariate_standard_normal_cdf(-d_1,nd_1,rho);
    let M2=bivariate_standard_normal_cdf(-d_2,nd_2,rho);
    LessDiv*N1+(-r*TDiv).exp()*(Div-K)*N2+LessDiv*M1-(-r*TCall).exp()*K*M2
}

//选择期权
pub fn chooser(S:f64,Kc:f64,Kp:f64,r:f64,sigma:f64,q:f64,T:f64,Tc:f64,Tp:f64)->f64{
    ///输入参数:
    /// S=初始股票价格
    /// Kc=看涨期权执行价格
    /// Kp=看跌期权执行价格
    /// r=无风险利率
    /// sigma=波动率
    /// Div=现金红利
    /// T=距离做出决策的时间
    /// Tc=距离看涨期权到期的时间
    /// Tp=距离看跌期权到期的时间
    ///
    let tol=1e-6;
    let mut lower=0.0;
    let mut upper=(q*Tc).exp()*(Kc+Kp);
    let mut guess=0.5*upper+0.5*lower;
    let mut flower=crate::black_scholes::european_call(lower,Kc,r,sigma,q,Tc-T)-crate::black_scholes::european_put(lower,Kp,r,sigma,q,Tp-T);
    //let mut flower=-(-r*(Tp-T)).exp()*Kp;
    let mut fupper=crate::black_scholes::european_call(upper,Kc,r,sigma,q,Tc-T)-crate::black_scholes::european_put(upper,Kp,r,sigma,q,Tp-T);
    let mut fguess=crate::black_scholes::european_call(guess,Kc,r,sigma,q,Tc-T)-crate::black_scholes::european_put(guess,Kp,r,sigma,q,Tp-T);
    while upper-lower>tol{
        if fupper*fguess<0.0{
            lower=guess;
            flower=fguess;
            guess=0.5*upper+0.5*lower;
            fguess=crate::black_scholes::european_call(guess,Kc,r,sigma,q,Tc-T)-crate::black_scholes::european_put(guess,Kp,r,sigma,q,Tp-T);
        }else{
            upper=guess;
            fupper=fguess;
            guess=0.5*upper+0.5*lower;
            fguess=crate::black_scholes::european_call(guess,Kc,r,sigma,q,Tc-T)-crate::black_scholes::european_put(guess,Kp,r,sigma,q,Tp-T);
        }
    }
    let Sstar=guess;

    let d1=((S/Sstar).ln()+(r-q+0.5*sigma*sigma)*T)/(sigma*T.sqrt());
    let d2=d1-sigma*T.sqrt();
    let d1c=((S/Kc).ln()+(r-q+0.5*sigma*sigma)*Tc)/(sigma*Tc.sqrt());
    let d2c=d1c-sigma*Tc.sqrt();
    let d1p=((S/Kp).ln()+(r-q+0.5*sigma*sigma)*Tp)/(sigma*Tp.sqrt());
    let d2p=d1p-sigma*Tp.sqrt();
    let rhoc=(T/Tc).sqrt();
    let rhop=(T/Tp).sqrt();
    let M1c=bivariate_standard_normal_cdf(d1,d1c,rhoc);
    let M2c=bivariate_standard_normal_cdf(d2,d2c,rhoc);
    let M1p=bivariate_standard_normal_cdf(-d1,-d1p,rhop);
    let M2p=bivariate_standard_normal_cdf(-d2,-d2p,rhop);
    (-q*Tc).exp()*S*M1c-(-r*Tc).exp()*Kc*M2c+(-r*Tp).exp()*Kp*M2p-(-q*Tp).exp()*S*M1p
}

pub fn call_on_max(S1:f64,S2:f64,K:f64,r:f64,sig1:f64,sig2:f64,rho:f64,q1:f64,q2:f64,T:f64)->f64{
    ///输入参数为
    /// S1=股票1的价格
    /// S2=股票2的价格
    /// K=执行价格
    /// r=无风险利率
    /// sig1=股票1的波动率
    /// sig2=股票2的波动率
    /// rho=相关系数
    /// q1=股票1的红利支付率
    /// q2=股票2的红利支付率
    /// T=到期时间
    let sigma=(sig2.powi(2)+sig1.powi(2)-2.0*rho*sig1*sig2).sqrt();
    let d1=((S1/S2).ln()+(q2-q1+0.5*sigma*sigma)*T)/(sigma*T.sqrt());
    let d2=d1-sigma*T.sqrt();
    let d11=((S1/K).ln()+(r-q1+0.5*sig1.powi(2))*T)/(sig1*T.sqrt());
    let d12=d11-sig1*T.sqrt();
    let d21=((S2/K).ln()+(r-q2+0.5*sig2.powi(2))*T)/(sig2*T.sqrt());
    let d22=d21-sig2*T.sqrt();
    let rho1=(sig1-rho*sig2)/sigma;
    let rho2=(sig2-rho*sig1)/sigma;
    let M1=bivariate_standard_normal_cdf(d11,d1,rho1);
    let M2=bivariate_standard_normal_cdf(d21,-d2,rho2);
    let M3=bivariate_standard_normal_cdf(-d12,-d22,rho);
    (-q1*T).exp()*S1*M1+(-q2*T).exp()*S2*M2+(-r*T).exp()*K*M3-(-r*T).exp()*K
}


