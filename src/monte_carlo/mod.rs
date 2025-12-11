//! 蒙特卡洛模拟定价模块

use super::*;
use crate::utils::cholesky_vec;

/// 欧式看涨期权蒙特卡洛定价（含Delta估计）
pub fn european_call_mc(S:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64,M:usize)->(f64,f64,f64){
    let LogS0=S.ln();
    let drift=(r-q-0.5*sigma.powi(2))*T;
    let SigSqrtT=sigma*T.sqrt();
    let UpChange:f64=(1.01_f64).ln();
    let DownChange:f64=(0.99_f64).ln();
    let mut SumCall=0.0;
    let mut SumCallChange=0.0;
    let mut SumPathwise=0.0;
    let mut rng=rand::rng();
    for i in 0..M{
        let rand_increment:f64=rng.sample(rand_distr::StandardNormal);
        let LogS:f64=LogS0+drift+SigSqrtT*rand_increment; //log S(T)的模拟值
        let CallV:f64=(LogS.exp()-K).max(0.0); //期权价值
        SumCall+=CallV; //期权价值求和
        let LogSu=LogS+UpChange; //log S_u(T)的模拟值
        let CallVu:f64=(LogSu.exp()-K).max(0.0); //期权价值
        let LogSd=LogS+DownChange; //log S_d(T)的模拟值
        let CallVd:f64=(LogSd.exp()-K).max(0.0); //期权价值
        SumCallChange+=CallVu-CallVd;  //期权价值之差
        if LogS.exp()>K{
            SumPathwise+=LogS.exp()/S;  //路径求和
        }
    }
    let CallV:f64=(-r*T).exp()*SumCall/M as f64;
    let Delta1:f64=(-r*T).exp()*SumCallChange/(M as f64*0.02*S);
    let Delta2:f64=(-r*T).exp()*SumPathwise/M as f64;
    (CallV,Delta1,Delta2)
}

/// GARCH模型下欧式看涨期权蒙特卡洛定价
pub fn european_call_garch_mc(S:f64,K:f64,r:f64,sigma0:f64,q:f64,T:f64,N:usize,kappa:f64,theta:f64,lambda:f64,M:usize)->(f64,f64){
    //输入参数：
    //S:股票初始价格
    //K:执行价格
    //sigma0:初始波动率
    //q:红利支付率
    //T:到期时间
    //N:时间区间数
    //kappa、theta、lambda:GARCH参数
    //M:模拟次数
    let deltaT:f64=T/N as f64;
    let sqrtT:f64=deltaT.sqrt();
    let a=kappa*theta;
    let b:f64=(1.0-kappa)*lambda;
    let c:f64=(1.0-kappa)*(1.0-lambda);
    let LogS0=S.ln();
    let mut SumCall=0.0;
    let mut SumCallSq=0.0;
    let mut rng=rand::rng();
    for i in 0..M{
        let mut LogS=LogS0;
        let mut Sigma=sigma0;
        for j in 0..N{
            let rand_increment:f64=rng.sample(rand_distr::StandardNormal);
            let y:f64=Sigma*rand_increment;
            LogS+=(r-q-0.5*Sigma.powi(2))*deltaT+sqrtT*y;
            Sigma=(a+b*y.powi(2)+c*Sigma.powi(2)).sqrt(); //更新波动率
        }
        let CallV:f64=(LogS.exp()-K).max(0.0);
        SumCall+=CallV;
        SumCallSq+=CallV.powi(2);
    }
    let CallV=(-r*T).exp()*SumCall/M as f64;
    let StdError=(-r*T).exp()*((SumCallSq-SumCall.powi(2)/M as f64)/(M*(M-1)) as f64).sqrt();
    (CallV,StdError)
}

/// 浮动执行价回望看涨期权蒙特卡洛定价（含标准误差）
pub fn floating_striking_call_mc_se(S:f64,r:f64,sigma:f64,q:f64,SMin:f64,T:f64,N:f64,M:f64)->(f64,f64){
    /// Monte Carlo Valuation of Path-Dependent Options
    /// 路径依赖期权的蒙特卡洛定价
    /// 输入参数 input parameters
    /// S = initial stock price 初始股票价格
    /// r = risk-free rate 无风险利率
    /// sigma = volatility 波动率
    /// q = dividend yield 红利支付率
    /// T = remaining time to maturity 剩余到期时间
    /// SMin = minimum during previous life of contract 从合约创设到当前时刻为止标的资产价格的最小值
    /// N = number of time period 期限个数
    /// M = number of simulations 模拟次数
    ///
    /// output results:
    /// call value
    /// standard error of call value
    let dt=T/N;
    let nudt=(r-q-0.5*sigma.powi(2))*dt;
    let sigsdt=sigma*dt.sqrt();
    let LogS0=S.ln();  //store ln of initial stock price
    let LogSMin0=SMin.ln();  //store ln of historical minimum
    let mut SumCall=0.0;   //initialize running total of values
    let mut SumCallSq=0.0; //initialize running total of squared values
    let mut rng=rand::rng();
    for i in 1..=M as usize{
        let mut LogS = LogS0;
        let mut LogSMin=LogSMin0;
        for j in 1..=N as usize{
            let increment:f64=rng.sample(StandardNormal);
            LogS+=nudt+sigsdt*increment;
            LogSMin=LogSMin.min(LogS);
        }
        let CallV=LogS.exp()-LogSMin.exp();
        SumCall+=CallV;
        SumCallSq+=CallV.powi(2);
    }
    let CallV=(-r*T).exp()*SumCall/M;
    let StdError=(-r*T).exp()*((SumCallSq-SumCall.powi(2)/M)/(M*(M-1.0))).sqrt();
    (CallV,StdError)
}

// 欧式篮子看涨期权蒙特卡洛定价
pub fn european_basket_call_mc(S:&[f64],K:f64,r:f64,cov:&Vec<Vec<f64>>,q:&[f64],w:&[f64],T:f64,M:usize)->Result<f64,String>{
    // ======================== 欧式篮子看涨期权蒙特卡洛定价函数 ========================
    /// 欧式篮子看涨期权蒙特卡洛定价
    ///
    /// # 参数
    /// - `S`: 初始股价向量（L 维）
    /// - `K`: 执行价格
    /// - `r`: 无风险利率
    /// - `cov`: 协方差矩阵（L×L）
    /// - `q`: 股息率向量（L 维）
    /// - `w`: 篮子权重向量（L 维）
    /// - `T`: 到期时间（年）
    /// - `M`: 模拟次数
    ///
    /// # 返回值
    /// Result<f64, String>: 期权价格（失败返回错误信息）

    // 1. 基础校验：维度一致性
    let l=S.len();
    if l==0{
        return Err("the kind of asserts cannot be 0".to_string());
    }
    if q.len()!=l || w.len()!=l || cov.len()!=l{
        return Err(format!("The dim is not right: asserts {},dividends {}, the line of cov matrix {}",q.len(),w.len(),cov.len()));
    }
    for row in cov{
        if row.len()!=l{
            return Err(format!("the cov matrix must be {}x{} matrix",l,l));
        }
    }
    if M==0{
        return Err("the number of simulation cannot be 0".to_string());
    }

    // 2. Cholesky 分解协方差矩阵
    let a=cholesky_vec(cov)?;

    // 3. 预计算均值（Mean）和乘数矩阵（Multiplier = sqrt(T) * a）
    let mut mean=vec![0.0;l];
    let mut multiplier=vec![vec![0.0;l];l];
    let sqrt_t=T.sqrt();

    for i in 0..l{
        mean[i]=S[i].ln()+(r-q[i]-0.5*cov[i][i])*T;
        for j in 0..l{
            multiplier[i][j]=sqrt_t*a[i][j];
        }
    }

    // 4.并行模拟（替换原单线程循环）大规模模拟（如 m > 1e6）可使用 rayon 库并行化循环，示例：
    let sum_call:f64=(0..M)
        .into_par_iter()
        .map(|_|{
            let mut rng=rand::rng();
            let z:Vec<f64>=(0..l).map(|_| rng.sample::<f64,StandardNormal>(StandardNormal)).collect();
            let mut basket_value=0.0;
            for i in 0..l{
                let mut logs=mean[i];
                for j in 0..l{
                    logs+=multiplier[i][j]*z[j];
                }
                basket_value+=logs.exp()*w[i];
            }
            (basket_value-K).max(0.0)
        })
        .sum();
    //5.计算贴现后的期权价格（平均价值 × 贴现因子）
    let discount_factor=(-r*T).exp();
    let option_value=discount_factor*sum_call/M as f64;
    Ok(option_value)
}

//Monte Carlo Valuation with an Antithetic Variate
//使用对偶变量法的浮动执行价回望看涨期权蒙特卡洛定价（含标准误差）
pub fn floating_striking_call_mc_av_se(S:f64,r:f64,sigma:f64,q:f64,SMin:f64,T:f64,N:usize,M:usize)->Result<(f64,f64),String>{
    /// Monte Carlo Valuation of Path-Dependent Options
    /// 路径依赖期权的蒙特卡洛定价
    /// 输入参数 input parameters
    /// S = initial stock price 初始股票价格
    /// r = risk-free rate 无风险利率
    /// sigma = volatility 波动率
    /// q = dividend yield 红利支付率
    /// T = remaining time to maturity 剩余到期时间
    /// SMin = minimum during previous life of contract 从合约创设到当前时刻为止标的资产价格的最小值
    /// N = number of time period 期限个数
    /// M = number of simulations 模拟次数
    ///
    /// output results:
    /// call value
    /// standard error of call value
    let dt=T/N as f64;
    let nudt=(r-q-0.5*sigma.powi(2))*dt;
    let sqrt_t=sigma*dt.sqrt();
    let LogS0=S.ln();
    let LogSMin0=SMin.ln();
    let mut SumCall=0.0;
    let mut SumCallSq=0.0;
    let mut rng=rand::rng();
    for i in 0..M{
        let mut LogS1=LogS0;
        let mut LogS2=LogS0;
        let mut LogSMin1=LogSMin0;
        let mut LogSMin2=LogSMin0;

        for j in 0..N{
            let z=rng.sample::<f64,StandardNormal>(StandardNormal);
            LogS1+=nudt+sqrt_t*z;
            LogS2+=nudt-sqrt_t*z;
            LogSMin1=LogSMin1.min(LogS1);
            LogSMin2=LogSMin2.min(LogS2);
        }
        let CallV=0.5*(LogS1.exp()-LogSMin1.exp())+0.5*(LogS2.exp()-LogSMin2.exp());
        SumCall+=CallV;
        SumCallSq+=CallV.powi(2);
    }
    let CallV=(-r*T).exp()*SumCall/M as f64;
    let stdError=(-r*T).exp()*((SumCallSq-SumCall.powi(2)/M as f64)/(M as f64*(M as f64-1.0)));

    return Ok((CallV,stdError))
}

// 用几何平均价格作为控制变量，定价含过去价格的算术平均价格看(亚式期权)
pub fn average_price_call_mc(S:f64,K:f64,r:f64,sigma:f64,q:f64,Avg:f64,TPast:f64,TFuture:f64,N:usize,M1:usize,M2:usize)->(f64,f64){
    ///用几何平均作为平均价格看涨期权定价中算术平均的控制变量，用事前样本估计贝塔值
    /// 输入变量：
    /// S=初始股票价格
    /// K=执行价格
    /// r=无风险利率
    /// sigma=波动率
    /// q=红利支付率
    /// Avg=过去的平均价格
    /// TPast=合约创设后持续时间
    /// N=时间区间个数
    /// M1=提前抽样的模拟次数
    /// M2=样本模拟和计算几何平均的模拟抽样次数
    let Kstar=(TFuture+TPast)*K/TFuture-TPast*Avg/TFuture;
    let dt=TFuture/N as f64;
    let nudt=(r-q-0.5*sigma.powi(2))*dt;
    let sigsdt=sigma*dt.sqrt();
    let disc=(-r*TFuture).exp();
    let LogS0=S.ln();

    //计算几何平均的已知均值
    let phi=crate::exotic_options::discrete_geom_average_price_call(S,Kstar,r,sigma,q,TFuture,N as f64);

    //进行提前抽样并估计回归中的β
    let mut Sumx=0.0; //算术平均期权价值求和
    let mut Sumx2=0.0; //算术平均期权价值平方求和
    let mut Sumy=0.0;  //几何平均期权价值求和
    let mut Sumy2=0.0; //几何平均期权价值平方求和
    let mut Sumxy=0.0; //乘积求和
    let mut rng=rand::rng();
    for i in 0..M1{
        let mut LogS=LogS0;
        let mut SumS=0.0;
        let mut SumLogS=0.0;
        for j in 0..N{
            LogS+=nudt+sigsdt*rng.sample::<f64,StandardNormal>(StandardNormal);
            SumS+=LogS.exp();
            SumLogS+=LogS;
        }
        let x=disc*((SumS/N as f64-Kstar).max(0.0));
        let y=disc*(((SumLogS/N as f64).exp()-Kstar).max(0.0));

        Sumx+=x;
        Sumx2+=x*x;
        Sumy+=y;
        Sumy2+=y*y;
        Sumxy+=x*y;
    }
    let beta=(M1 as f64*Sumxy-Sumx*Sumy)/(M1 as f64 *Sumy2-Sumy*Sumy);

    //计算样本算术平均和几何平均
    let mut Sumx=0.0;
    let mut Sumy=0.0;
    for i in 0..M2{
        let mut LogS=LogS0;
        let mut SumS=0.0;
        let mut SumLogS=0.0;
        for j in 1..N{
            LogS+=nudt+sigsdt*rng.sample::<f64,StandardNormal>(StandardNormal);
            SumS+=LogS.exp();
            SumLogS+=LogS;
        }
        let x=disc*((SumS/N as f64-Kstar).max(0.0));
        let y=disc*(((SumLogS/N as f64).exp()-Kstar).max(0.0));

        Sumx+=x;
        Sumy+=y;
    }
    let CallV=(TFuture/(TFuture+TPast))*(Sumx/M2 as f64+beta*(phi-Sumy/M2 as f64));
    (CallV,beta)
}