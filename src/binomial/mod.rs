//! 二叉树模型定价模块

use super::*;

/// 欧式看涨期权二叉树定价
pub fn european_call_binomial(S:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64,N:usize)->f64{
    let dt=T/N as f64; //时间区间长度
    let u=(sigma*dt.sqrt()).exp(); //上升步长
    let d=1.0/u; //下降步长
    let pu=(((r-q)*dt).exp()-d)/(u-d); //上升概率
    let pd=1.0-pu;  //下降概率
    let u2=u*u;

    //计算底部节点（即每次都下降达到的节点）处的股票价格、到达该节点的概率以及公式(5.4)中的第一项
    let mut S=S*d.powi(N as i32);
    let mut prob=pd.powi(N as i32);
    let mut CallV=prob*(S-K).max(0.0);
    for i in 1..=N{
        S*=u2;
        prob*=(pu/pd)*(N-i+1) as f64/i as f64;
        CallV+=prob*(S-K).max(0.0);
    }
    (-r*T).exp()*CallV

}

/// 美式看跌期权二叉树定价
pub fn american_put_binomial(S0:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64,N:usize)->f64{
    //输入参数为
    //S0:初始股票价格
    //K:执行价格
    //r:无风险利率
    //sigma:波动率
    //q:红利支付率
    //T:到期时间
    //N:时间区间个数
    let dt=T/N as f64;  //时间区间长度
    let u=(sigma*dt.sqrt()).exp();  //上升步长
    let d=1.0/u;  //下降步长
    let pu=(((r-q)*dt).exp()-d)/(u-d); //上升概率
    let pd=1.0-pu;  //下降概率
    let discount=(-r*dt).exp();
    let dpu=discount*pu; //贴现上升概率
    let dpd:f64=discount*pd; //贴现下跌概率
    let u2=u*u;

    let mut S=S0*d.powi(N as i32); //底部节点的股票价格
    let mut PutV=vec![0.0;N+1];
    PutV[0]=(K-S).max(0.0); //底部节点看跌期权价值
    for i in 1..=N{
        S*=u2;
        PutV[i]=(K-S).max(0.0);
    }

    for i in (0..N).rev(){ //沿时间倒回到0时
        let mut S_cur=S0*d.powi(i as i32);  //底部节点的股票价格
        PutV[0]=(K-S_cur).max(dpd*PutV[0]+dpu*PutV[1]);
        for j in 1..=i{
            S_cur*=u2;
            PutV[j]=(K-S_cur).max(dpd*PutV[j]+dpu*PutV[j+1]);
        }
    }
    PutV[0]

}

/// 美式看跌期权二叉树定价（含Delta和Gamma）
pub fn american_put_binomial_delta_gamma(S0:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64,N:usize)->(f64,f64,f64){
    //输入参数为
    //S0:初始股票价格
    //K:执行价格
    //r:无风险利率
    //sigma:波动率
    //q:红利支付率
    //T:到期时间
    //N:时间区间个数
    //返回(put value,delta,gamma)
    let dt=T/N as f64;
    let NewN=N+2;
    let u=(sigma*dt.sqrt()).exp();
    let d=1.0/u;
    let pu=(((r-q)*dt).exp()-d)/(u-d);
    let pd=1.0-pu;
    let dpu=(-r*dt).exp()*pu;
    let dpd=(-r*dt).exp()*pd;
    let u2=u*u;

    let mut S=S0*d.powi(NewN as i32);
    let mut PutV=vec![0.0;NewN+1];

    let mut S=S0*d.powi(NewN as i32);
    PutV[0]=(K-S).max(0.0);
    for i in (2..=NewN-1).rev(){
        let mut S_cur=S0*d.powi(i as i32);
        PutV[0]=(K-S_cur).max(dpd*PutV[0]+dpu*PutV[1]);
        for j in 1..=i{
            S_cur*=u2;
            PutV[j]=(K-S_cur).max(dpd*PutV[j]+dpu*PutV[j+1]);
        }
    }
    let Su=S0*u2;  //高股票价格
    let Sd=S0/u2;  //低股票价格

    let DeltaU=(PutV[2]-PutV[1])/(Su-S0); //上中点德尔塔
    let DeltaD=(PutV[1]-PutV[0])/(S0-Sd); //下中点德尔塔

    let distance=S0*(u*u-d*d); //Su和Sd的距离
    let delta=(PutV[2]-PutV[0])/distance;
    let gamma=2.0*(DeltaU-DeltaD)/distance;

    (PutV[1],delta,gamma)
}

/// 美式价差看跌期权二叉树定价
pub fn american_spread_put_binomial(S:[f64;2],K:f64,r:f64,sigma:[f64;2],rho:f64,q:[f64;2],T:f64,N:f64)->f64{
    /// Binomial Valuation of American Spread Options
    /// 美式价差看跌期权二叉树定价
    ///
    /// # 参数
    /// - `S`: 2-vector of initial stock prices 初始股价数组 [S1, S2]
    /// - `K`: striking price 执行价格
    /// - `r`: risk-free ratio 无风险利率
    /// - `sigma`: 2-vector of volatilities 波动率数组 [σ1, σ2]
    /// - `rho`: correlation 相关系数
    /// - `q`: 2-vector of dividened yields 股息率数组 [q1, q2]
    /// - `T`: time to maturity 到期时间（年）
    /// - `N`: number of periods in binomial tree 二叉树期数
    ///
    /// # 返回值
    /// 期权价格
    let dt = T/N;
    let mut nu=vec![0.0;2];
    let mut logu=vec![0.0;2];
    let mut u=vec![0.0;2];
    let mut d=vec![0.0;2];
    let mut p=vec![0.0;2];
    let mut u2=vec![0.0;2];

    //parameters of each stock
    for i in 0..2{
        nu[i]=r-q[i]-sigma[i].powi(2)/2.0;
        logu[i]=(sigma[i].powi(2)*dt+nu[i].powi(2)*dt.powi(2)).sqrt();
        u[i]=logu[i].exp();
        d[i]=1.0/u[i];
        p[i]=0.5*(1.0+nu[i]*dt/logu[i]);
        u2[i]=u[i]*u[i];
    }

    let num=rho*sigma[0]*sigma[1]*dt+nu[0]*nu[1]*dt*dt;
    let constant=num/(logu[0]*logu[1]);
    let pud=(p[0]-p[1])/2.0+(1.0-constant)/4.0;
    let pdu=pud-p[0]+p[1];
    let puu=p[0]-pud;
    let pdd=1.0-pud-pdu-puu;

    let disc=(-r*dt).exp(); //one period discount factor
    let dpuu=disc*puu;
    let dpud=disc*pud;
    let dpdu=disc*pdu;
    let dpdd=disc*pdd;

    //初始化股票价格和期权价值矩阵
    let mut stock=vec![vec![0.0;N as usize+1];2];
    let mut putv=vec![vec![0.0;N as usize +1];N as usize +1];

    for i in 0..2{
        stock[i][0]=S[i]*d[i].powi(N as i32);
        for j in 1..=(N as usize){
            stock[i][j]=stock[i][j-1]*u2[i];
        }
    }
    //the intrinsic value of option of last term
    //计算最后一起的期权内在价值
    for j in 0..=(N as usize){
        for h in 0..(N as usize){
            putv[j][h]=(K-stock[0][j]+stock[1][h]).max(0.0);
        }
    }
    //倒退各期期权价值
    for i in (0..N as usize).rev(){
        //计算当前期的股票价值
        for x in 0..2{
            stock[x][0]=S[x]*d[x].powi(i as i32);
            for j in 1..=i{
                stock[x][j]=stock[x][j-1]*u2[x];
            }
        }
        //计算当前期的期权价值（提前行权判断）
        for j in 0..=i {
            for h in 0..=i {
                let intrinsic_v = K - stock[0][j] + stock[1][h];
                let disc_v = dpdd * putv[j][h]
                    + dpdu * putv[j][h + 1]
                    + dpud * putv[j + 1][h]
                    + dpuu * putv[j + 1][h + 1];
                putv[j][h] = intrinsic_v.max(disc_v);
            }
        }
    }
    putv[0][0]
}
pub fn american_put_binomial_bs(S0:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64,N:usize)->f64{

    let dt=T/N as f64;
    let u=(sigma*dt.sqrt()).exp();
    let d=1.0/u;
    let pu=(((r-q)*dt).exp()-d)/(u-d);
    let dpu=(-r*dt).exp()*pu;
    let dpd=(-r*dt).exp()*(1.0-pu);
    let u2=u*u;

    let mut PutV=vec![0.;N];
    let mut S=S0*d.powf(N as f64-1.0);
    PutV[0]=crate::black_scholes::european_put(S,K,r,sigma,q,dt).max(K-S);
    for j in 1..N{
        S*=u2;
        PutV[j]=crate::black_scholes::european_put(S,K,r,sigma,q,dt).max(K-S);
    }

    for i in (0..N-1).rev(){
        S=S0*d.powf(i as f64);
        PutV[0]=(dpd*PutV[0]+dpu*PutV[1]).max(K-S);
        for j in 1..=i{
            S=S*u2;
            PutV[j]=(dpd*PutV[j]+dpu*PutV[j+1]).max(K-S);
        }
    }
    PutV[0]
}

pub fn American_Put_Binomial_BS_RE(S:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64,N:usize)->Result<f64,&'static str>{
    if N%2!=0{
        return Err("the number of periods N must be divisible by 2");
    }
    let y2=american_put_binomial_bs(S,K,r,sigma,q,T,N);
    let y1=american_put_binomial_bs(S,K,r,sigma,q,T,N/2);
    Ok(2.0*y2-y1)
}