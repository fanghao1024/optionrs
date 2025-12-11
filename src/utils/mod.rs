//! 通用工具函数模块

use super::*;
use owens_t;

//二元正态分布累计函数
pub fn bivariate_standard_normal_cdf(a:f64,b:f64,rho:f64)->f64{
    let standard_normal = Normal::new(0.0, 1.0).unwrap();
    let phi_a = standard_normal.cdf(a);
    let phi_b = standard_normal.cdf(b);

    let p_gt_a_gt_b = owens_t::biv_norm(a, b, rho);

    // P(<a, <b) = P(<a) + P(<b) - 1 + P(>a, >b)
    phi_a + phi_b - 1.0 + p_gt_a_gt_b
}

/// 计算数组的指定百分位数
fn calc_percentage(data:&mut [f64],pct:f64)->Result<f64,&'static str>{
    if data.is_empty(){
        return Err("Data is empty");
    }
    if pct<0.0 || pct>1.0{
        return Err("Percentile must be between 0 and 1");
    }
    data.sort_by(|a,b| a.partial_cmp(b).unwrap());
    let n=data.len() as f64;
    let rank=pct*(n-1.0);
    let lower_rank=rank.floor() as usize;
    let upper_rank=rank.ceil() as usize;
    // 整数秩直接返回对应值
    if lower_rank==upper_rank{
        Ok(data[lower_rank])
    }else{
        // 线性插值计算
        let fraction=rank-lower_rank as f64;
        Ok(data[lower_rank]*(1.0-fraction)+data[upper_rank]*fraction)
    }
}

pub fn Simulated_Delta_Hedge_Profit_Forward(F0:f64,K:f64,r:f64,sigma:f64,T:f64,Tf:f64,mu:f64,M:usize,N:usize,Pct:f64)->Result<f64,&'static str>{
    /// 模拟Delta对冲策略的利润，并返回指定百分位数
    /// 参数：
    /// - F0: 初始远期价格
    /// - K: 行权价格
    /// - r: 无风险利率
    /// - sigma: 波动率
    /// - T: 期权到期时间（年）
    /// - Tf: 远期合约到期时间（年）
    /// - mu: 远期价格预期升值率
    /// - M: 模拟次数
    /// - N: 时间步数
    /// - Pct: 百分位（0~1之间）
    /// 返回：成功返回指定百分位数利润，失败返回错误信息
    if T<=0.0 || Tf<T{
        return Err("T must be positive and Tf>=T");
    }
    if M==0 || N==0{
        return Err("M(simulations) and N(time steps) must be > 0");
    }
    if sigma<0.0{
        return Err("sigma cannot be negative");
    }
    let dt=T/N as f64;
    let sig_sqrdt=sigma*dt.sqrt();
    let drift=(mu-0.5*sigma*sigma)*dt;

    let log_F0=F0.ln();
    let P0T=E.powf(-r*Tf);
    let forwards0=crate::generic::black_call_delta(F0,K,P0T,sigma,T); //初始德尔塔头寸
    let cash=crate::generic::black_call_2(F0,K,P0T,sigma,T); //初始期权价格

    let mut profit=vec![0.0;M];
    let mut rng=rand::rng();

    for i in 0..M{
        let mut f=F0;
        let mut log_f:f64=log_F0;
        let mut forwards=forwards0;
        let mut forward_gains=0.0;

        for j in 1..N{
            let increment_random:f64=rng.sample(StandardNormal);
            log_f+=drift+sig_sqrdt*increment_random;
            let new_f=log_f.exp();

            forward_gains+=forwards*(new_f-f);
            f=new_f;

            let remaining_tf=Tf-j as f64*dt;
            let p=E.powf(-r*remaining_tf);
            let remaining_t=T-j as f64*dt;
            forwards=crate::generic::black_call_delta(f,K,p,sigma,remaining_t)
        }
        let increment_random:f64=rng.sample(StandardNormal);
        log_f=log_f+drift+sig_sqrdt*increment_random;
        let new_f=log_f.exp();
        forward_gains+=forwards*(new_f-f);

        let hedge_value=cash*E.powf(-r*T)+forward_gains;
        let option_value=E.powf(-r*(Tf-T))*(new_f-K).max(0.0);

        profit[i]=hedge_value-option_value;
    }
    calc_percentage(&mut profit,Pct)
}

pub fn cholesky(cov:&[&[f64]])->Result<Vec<Vec<f64>>,String>{
    /// Cholesky 分解（乔列斯基分解）
    /// 将对称正定的协方差矩阵分解为下三角矩阵 L，满足 cov = L * L^T
    ///
    /// # 参数
    /// - `cov`: 二维切片形式的 L×L 对称正定协方差矩阵（行优先存储）
    ///
    /// # 返回值
    /// - `Result<Vec<Vec<f64>>, String>`: 成功返回下三角矩阵 L；失败返回错误信息
    ///
    /// # 错误场景
    /// - 矩阵非方阵
    /// - 矩阵维度为 0
    /// - 平方根内为负数（矩阵非正定）
    /// - 对角线元素为 0（导致除零错误）
    let l=cov.len();
    if l==0{
        return Err("The dim of matrix cannot be 0".to_string());
    }
    for row in cov{
        if row.len()!=l{
            return Err(format!("Cov matrix must be {}x{} marix, and the length of the current row is {}.",l,l,row.len()));
        }
    }

    let mut a=vec![vec![0.0;l];l];

    for i in 0..l{
        let sum_sq:f64=(0..i).map(|h| a[i][h]*a[i][h]).sum();

        let diag_val=cov[i][i]-sum_sq;
        if diag_val<(-f64::EPSILON){
            return Err(format!("矩阵非正定：第{}行对角元素计算时平方根为负数(值:{:.6})",i+1,diag_val));
        }
        a[i][i]=diag_val.max(0.0).sqrt();

        if a[i][i].abs()<f64::EPSILON{
            return Err(format!(
                "第{}行对角元素为0，无法完成分解（可能矩阵奇异）",
                i+1
            ));
        }

        for j in (i+1)..l{
            let sum_pr:f64=(0..i).map(|h| a[i][h]*a[j][h]).sum();
            a[j][i]=(cov[i][j]-sum_pr)/a[i][i];
        }
    }
    Ok(a)
}

pub fn cholesky_vec(cov:&Vec<Vec<f64>>)->Result<Vec<Vec<f64>>,String>{
    let slice:Vec<&[f64]>=cov.iter().map(|row|row.as_slice()).collect();
    cholesky(&slice)
}

pub fn crank_nicolson(a:&[f64],y:&[f64],l:usize,z1:f64,b1:f64,zl:f64,bl:f64)->Result<Vec<f64>,&'static str>{
    /// 实现Crank-Nicolson算法的核心函数
    ///
    /// # 参数说明
    /// - a: 4维系数向量
    /// - y: 某时间点的函数值向量，长度必须等于L
    /// - l: 空间网格点数量（无符号整数，至少为2）
    /// - z1: 底部边界条件常数
    /// - b1: 底部边界条件系数
    /// - zl: 顶部边界条件常数
    /// - bl: 顶部边界条件系数
    ///
    /// # 返回值
    /// - Ok(Vec<f64>): 成功时返回前一时间点的函数值向量
    /// - Err(&str): 失败时返回错误信息（输入不合法/除零错误）
    if a.len()!=4{
        return Err("the number of periods must be 4");
    }
    if y.len()!=l{
        return Err(format!("the length of y must be L(now is {})",y.len()).leak());
    }
    if l<2{
        return Err("l must be greater than 2");
    }
    let mut u=vec![0.0;l];
    let mut b_coeff=vec![0.0;l];
    let mut z=vec![0.0;l];
    let mut c=vec![0.0;l];

    //初始化边界条件
    u[0]=z1;
    b_coeff[0]=b1;

    //前向计算u和b_coeff
    for j in 1..l-1{
        z[j]=a[3]*y[j]+a[1]*y[j+1]+a[2]*y[j-1];

        let denominator=a[0]-a[2]*b_coeff[j-1];
        if denominator==0.0{
            return Err("error when calculating u/b");
        }
        u[j]=(a[2]*u[j-1]+z[j])/denominator;
        b_coeff[j]=a[1]/denominator;
    }

    //计算顶部边界的期权值：c(L)=(zL + bL*u(L-1))/(1 - bL*b(L-1))
    let numerator_c_last=zl+bl*u[l-2];
    let denominator_c_last=1.0-bl*b_coeff[l-2];
    if denominator_c_last==0.0{
        return Err("error when computing C[L-1]");
    }
    c[l-1]=numerator_c_last/denominator_c_last;

    //反向计算C
    for j in (0..l-1).rev(){
        c[j]=u[j]+b_coeff[j]*c[j+1];
    }
    Ok(c)
}