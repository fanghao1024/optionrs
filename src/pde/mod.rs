use super::*;
use crate::utils::crank_nicolson;


pub fn european_call_crank_nicolson(S0:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64,N:usize,M:usize,dist:f64)->Result<f64,&'static str>{
    /// 欧式看涨期权Crank-Nicolson定价函数
    ///
    /// # 参数说明
    /// - S0: 初始股票价格
    /// - K: 执行价格
    /// - r: 无风险利率（年化）
    /// - sigma: 波动率（年化）
    /// - q: 股息率（年化）
    /// - T: 到期时间（年）
    /// - N: 时间步数
    /// - M: 对数价格网格上下各M个点
    /// - dist: 对数价格网格上下边界距离log(S0)的距离
    ///
    /// # 返回值
    /// - Ok(f64): 期权定价结果（中间节点值）
    /// - Err(&str): 错误信息（输入非法/计算错误）

    //1.检验输入的合法性
    if S0<=0.0 || K<=0.0 || sigma<0.0 || T<0.0 || N==0 || M==0 ||dist<0.0{
        return Err("Illegal parameters!");
    }

    //2.计算网格和时间步参数
    let l=2*M+1; //纵轴方向的总点数
    let dt=T/N as f64; //时间方向（横轴）网格间的间隔，时间步长
    let dx=dist/M as f64; //纵轴方向网格间的间隔，空间步长
    let dx2=dx*dx;
    let u=dx.exp(); //上升参数，与二叉树模型相同
    let sig2=sigma*sigma;
    let nu=r-q-0.5*sig2;

    //3.计算价格边界
    let St=S0*dist.exp();//顶部节点股票价格
    let Sb=S0*(-dist).exp(); //底部节点股票价格

    //4.计算Crank-Nicolson系数a
    let mut a=vec![0.0;4];
    a[0]=(r/2.0)+(1.0/dt)+sig2/(2.0*dx2);
    a[1]=(sig2/(4.0*dx2))+nu/(4.0*dx);
    a[2]=(sig2/(4.0*dx2))-nu/(4.0*dx);
    a[3]=(1.0/dt)-(r/2.0)-(sig2/(2.0*dx2));

    //5.初始化到期时刻T的期权价值向量y
    let mut y=vec![0.0;l];
    let mut S=Sb;
    y[0]=(S-K).max(0.0);

    for j in 1..l{
        S*=u;
        y[j]=(S-K).max(0.0);
    }

    //6.定义边界条件
    let z1=0.0;
    let b1=1.0;
    let zl=St-St/u;
    let bl=1.0;

    //7.计算倒数第二个时间点的期权价值
    let mut Call_V=crank_nicolson(&a,&y,l,z1,b1,zl,bl)?;

    //8.回退到时间点0
    if N>1{
        for _ in 0..(N-1){
            Call_V=crank_nicolson(&a,&Call_V,l,z1,b1,zl,bl)?;
        }
    }
    Ok(Call_V[M])

}

pub fn down_and_out_call_crank_nicolson(
    S0:f64,
    K:f64,
    r:f64,
    sigma:f64,
    q:f64,
    T:f64,
    N:usize,
    M:usize,
    dist:f64,
    bar:f64
)->Result<f64,&'static str>{
    /// 向下敲出（Down-and-Out）看涨期权Crank-Nicolson定价函数
    /// 严格对齐原VBA逻辑，适配敲出障碍边界条件
    ///
    /// # 参数说明
    /// - S0: 初始股票价格
    /// - K: 执行价格
    /// - r: 无风险利率（年化）
    /// - sigma: 波动率（年化）
    /// - q: 股息率（年化）
    /// - T: 到期时间（年）
    /// - N: 时间步数
    /// - M: 初始对数价格网格顶部半点数（用于初始dx估算）
    /// - dist: 初始对数价格网格顶部距离log(S0)的距离
    /// - bar: 敲出障碍价格（要求Bar < S0）
    ///
    /// # 返回值
    /// - Ok(f64): 敲出期权定价结果（中间节点值）
    /// - Err(&str): 错误信息（输入非法/计算错误）

    //1.输入的合法性检查
    if S0<=0.0 || K<=0.0 || sigma<0.0 || T<0.0 || N==0 || M==0 ||dist<0.0 || bar<0.0{
        return Err("Illegal parameters!");
    }

    if bar>=S0{
        return Err("Barrier price must be greater than initial stock price");
    }

    //2.自适应调整空间步长dx(核心逻辑：匹配障碍价格的网格对齐）
    let mut dx=dist/M as f64; //初步dx估算
    let dist_bot=S0.ln()-bar.ln();//计算log(S0)到log(bar)的距离
    let num_bot_steps=(dist_bot/dx).ceil() as usize;
    if num_bot_steps==0{
        return Err("Number steps between S0 and botton cannot be 0");
    }
    dx=dist_bot/(num_bot_steps as f64);

    //计算顶部步数
    let num_top_steps=(dist/dx).ceil() as usize;
    if num_top_steps==0{
        return Err("Number steps between S0 and top cannot be 0");
    }
    let dist_top=num_top_steps as f64 *dx;
    let l=num_bot_steps+num_top_steps+1 ;
    if l<2{
        return Err("l must be greater than 2");
    }

    //3.计算时间不长和核心参数
    let dt=T/N as f64;
    let dx2=dx*dx;
    let u=dx.exp();
    let sig2=sigma*sigma;
    let nu=r-q-0.5*sig2;

    //4.顶部节点的股价
    let St=S0*dist_top.exp();

    //5.计算Crank-Nicolson系数a
    let mut a=[0.0;4];
    a[0]=(r/2.0)+(1.0/dt)+sig2/(2.0*dx2);
    a[1]=(sig2/(4.0*dx2))+nu/(4.0*dx);
    a[2]=(sig2/(4.0*dx2))-nu/(4.0*dx);
    a[3]=(1.0/dt)-(r/2.0)-(sig2/(2.0*dx2));

    //6.初始化到时刻T的期权价值向量y
    let mut y=vec![0.0;l];
    let mut S=bar;
    y[0]=(S-K).max(0.0);

    for j in 1..l{
        S*=u;
        y[j]=(S-K).max(0.0);
    }

    //7.敲出期权的边界条件（核心区别：底部b1=0,而非欧式的1）
    let z1=0.0;
    let b1=0.0;
    let zl=St-St/u;
    let bl=1.0;

    //8.计算倒数第二个时点的期权价值
    let mut CallV=crank_nicolson(&a,&y,l,z1,b1,zl,bl)?;

    //9.回退到时间0
    if N>1{
        for _ in 0..N-1{
            CallV=crank_nicolson(&a,&CallV,l,z1,b1,zl,bl)?;
        }
    }
    // 10. 返回初始股价对应节点的价值：VBA CallV(NumBotSteps+1) → Rust call_v[NumBotSteps]
    if num_bot_steps >= CallV.len() {
        return Err("目标节点索引超出网格范围");
    }
    Ok(CallV[num_bot_steps])
}