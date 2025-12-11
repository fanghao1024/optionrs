//! 奇异期权定价模块

use super::*;
use crate::utils::bivariate_standard_normal_cdf;

/// 远期生效看涨期权
pub fn forward_start_call(S:f64,r:f64,sigma:f64,q:f64,Tset:f64,TCall:f64)->f64{
    let d_1=(r-q+0.5*sigma*sigma)*(TCall-Tset).sqrt()/sigma;
    let d_2=d_1-sigma*(TCall-Tset).sqrt();
    let normal=Normal::new(0.0, 1.0).unwrap();
    (-q*TCall).exp()*S*normal.cdf(d_1)-(-q*Tset-r*(TCall-Tset)).exp()*S*normal.cdf(d_2)

}

//下跌失效看涨期权
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

//离散抽样几何平均价格看涨期权
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


fn find_sstar_call(Kc:f64,Ku:f64,r:f64,sigma:f64,q:f64,Tc:f64,Tu:f64)->f64{
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

//看涨期权上的看涨期权
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

//看跌期权上的看涨期权
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

//含红利的美式看涨期权
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

//最大值看涨期权
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


