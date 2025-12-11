//! Black-Scholes模型及相关希腊字母计算

use super::*;

// Black-Scholes看涨期权定价模型
///
/// # 参数
/// - S: 标的资产当前价格
/// - K: 期权行权价
/// - r: 无风险利率（年化）
/// - sigma: 标的资产波动率（年化）
/// - q: 股息收益率（年化）
/// - T: 到期时间（年）
///
/// # 返回值
/// 看涨期权的理论价格
///
/// # 公式
/// C = S*e^(-qT)*N(d1) - K*e^(-rT)*N(d2)
/// d1 = [ln(S/K) + (r - q + σ²/2)T] / (σ√T)
/// d2 = d1 - σ√T
///
/// # 示例（用户可直接运行）
/// ```
/// use optionrs::black_scholes::european_call;
/// // 自定义近似比较（精度 1e-4，适配不同环境的浮点误差）
/// fn assert_approx_eq(a: f64, b: f64, eps: f64) {
///     assert!((a - b).abs() < eps, "{} != {} (eps: {})", a, b, eps);
/// }
/// // 经典场景：标的50.0，行权价40，无风险利率5%，波动率30%，无股息2%，到期2年
/// let price = european_call(50.0, 40.0, 0.05, 0.3, 0.02, 2.0);
///
/// assert_approx_eq(price,14.48306,1e-4)
/// ```
pub fn european_call(S:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64)->f64{
    //处理边界情况
    if T<=0.0{
        // 到期时期权价值为内在价值
        return (S-K).max(0.0);
    }

    if sigma == 0.0{
        // 波动率为0时的特殊情况
        let instrinsic_value=(S*(-q*T).exp()-K*(-r*T).exp()).max(0.0);
        return instrinsic_value;
    }

    let sqrt_t=T.sqrt();
    let d1=(S.ln()-K.ln()+(r-q+0.5*sigma.powi(2))*T)/(sigma*sqrt_t);
    let d2=d1-sigma*sqrt_t;

    //Normal::new(mean, std_dev) 创建一个正态分布实例。
    // 由于参数可能无效（如标准差为负），它返回一个 Result，我们用 unwrap() 来获取成功的结果
    // （在生产代码中，你应该更谨慎地处理 Result）
    let standard_norm=Normal::new(0.0,1.0).unwrap();
    //调用 standard_normal.cdf(x) 即可计算出 P (X ≤ x) 的值
    let n_d1=standard_norm.cdf(d1);
    let n_d2=standard_norm.cdf(d2);

    S*(-q*T).exp()*n_d1-K*(-r*T).exp()*n_d2
}

/// Black-Scholes看跌期权定价模型
///
/// # 参数
/// - s: 标的资产当前价格
/// - k: 期权行权价
/// - r: 无风险利率（年化）
/// - sigma: 标的资产波动率（年化）
/// - q: 股息收益率（年化）
/// - t: 到期时间（年）
///
/// # 返回值
/// 看跌期权的理论价格
///
/// # 公式
/// P = K*e^(-rT)*N(-d2) - S*e^(-qT)*N(-d1)
/// d1 = [ln(S/K) + (r - q + σ²/2)T] / (σ√T)
/// d2 = d1 - σ√T
pub fn european_put(S:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64)->f64{
    if T<=0.0{
        return (K-S).max(0.0);
    }
    if sigma == 0.0{
        let instrinsic_value=K*(-r*T).exp()-S*(-q*T).exp();
        return instrinsic_value;
    }
    let sqrt_t=T.sqrt();
    let d1=(S.ln()-K.ln()+(r-q+0.5*sigma.powi(2))*T)/(sigma*sqrt_t);
    let d2=d1-sigma*sqrt_t;

    //Normal::new(mean, std_dev) 创建一个正态分布实例。
    // 由于参数可能无效（如标准差为负），它返回一个 Result，我们用 unwrap() 来获取成功的结果
    // （在生产代码中，你应该更谨慎地处理 Result）
    let standard_norm=Normal::new(0.0,1.0).unwrap();
    //调用 standard_normal.cdf(x) 即可计算出 P (X ≤ x) 的值
    let n_nd1=standard_norm.cdf(-d1);
    let n_nd2=standard_norm.cdf(-d2);

    K*(-r*T).exp()*n_nd2-S*(-q*T).exp()*n_nd1
}

pub fn black_scholes_call_delta(S:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64)->f64{
    /// Black-Scholes看涨期权Delta计算
    ///
    /// # 参数
    /// - s: 标的资产当前价格
    /// - k: 期权行权价
    /// - r: 无风险利率（年化）
    /// - sigma: 标的资产波动率（年化）
    /// - q: 股息收益率（年化）
    /// - t: 到期时间（年）
    ///
    /// # 返回值
    /// 看涨期权的Delta值，范围在[0, 1]之间
    ///
    /// # 公式
    /// Δ = e^(-qT) * N(d1)
    /// d1 = [ln(S/K) + (r - q + σ²/2)T] / (σ√T)

    if T<=0.0{
        return if S>K {1.0}else{0.0}
    }
    if sigma == 0.0{
        return if S*(-q*T).exp()>K*(-r*T).exp(){
            (-q*T).exp()
        }else{
            0.0
        };
    }
    let sqrt_t=T.sqrt();
    let d1=(S.ln()-K.ln()+(r-q+0.5*sigma.powi(2))*T)/(sigma*sqrt_t);

    let standard_norm=Normal::new(0.0,1.0).unwrap();
    let n_d1=standard_norm.cdf(d1);
    (-q*T).exp()*n_d1
}

/// Black-Scholes看涨期权Gamma计算
///
/// # 参数
/// - S: 标的资产当前价格
/// - K: 期权行权价
/// - r: 无风险利率（年化）
/// - sigma: 标的资产波动率（年化）
/// - q: 股息收益率（年化）
/// - T: 到期时间（年）
///
/// # 返回值
/// 看涨期权的Gamma值
///
/// # 公式
/// Γ = e^(-qT) * N'(d1) / (S * σ * √T)
/// 其中 N'(d1) = e^(-d1²/2) / √(2π)
/// d1 = [ln(S/K) + (r - q + σ²/2)T] / (σ√T)
///
/// # 金融意义
/// Gamma衡量Delta对标的资产价格变化的敏感性，是期权凸性的度量
pub fn black_scholes_call_gamma(S:f64,K:f64,r:f64,sigma:f64,q:f64,T:f64)->f64{
    // 处理边界情况
    if T <= 0.0 || sigma <= 0.0 || S <= 0.0 {
        return 0.0;
    }

    // 计算d1
    let sqrt_t = T.sqrt();
    let d1 = (S.ln() - K.ln() + (r - q + 0.5 * sigma.powi(2)) * T) / (sigma * sqrt_t);

    // 计算标准正态分布的概率密度函数值 N'(d1)
    let standard_norm=Normal::new(0.0,1.0).unwrap();
    let nd1 = standard_norm.pdf(d1);

    // Black-Scholes看涨期权Gamma公式
    (-q * T).exp() * nd1 / (S * sigma * sqrt_t)
}

/// 计算Black-Scholes看涨期权的隐含波动率
/// 参数:
/// - S: 初始股票价格
/// - K: 行权价格
/// - r: 无风险利率
/// - q: 股息收益率
/// - T: 到期时间（年）
/// - CallPrice: 看涨期权市场价格
///
/// 返回: Result<f64, String> - 成功时返回隐含波动率，错误时返回错误信息
pub fn black_scholes_call_implied_vol(S:f64,K:f64,r:f64,q:f64,T:f64,CallPrice:f64)->Result<f64,String> {
    if CallPrice<S*(-q*T).exp()-K*(-r*T).exp(){
        return Err("Option price violates the arbitrage bound.".to_string());
    }

    let tol=1e-6;
    let mut lower=0.0;
    let mut upper=1.0;

    let mut flower=european_call(S,K,r,lower,q,T)-CallPrice;
    let mut fupper:f64=european_call(S,K,r,upper,q,T)-CallPrice;

    while fupper<0.0{
        upper*=2.0;
        fupper=european_call(S,K,r,upper,q,T)-CallPrice;

        // 防止无限循环
        if upper > 100.0 {
            return Err("Unable to find valid upper bound for volatility.".to_string());
        }
    }

    //二分法求解
    let mut guess=(upper+lower)/2.0;
    let mut fguess:f64=european_call(S,K,r,guess,q,T)-CallPrice;

    let max_iter=1000;
    let mut iter=0;

    while (upper-lower)>tol && iter<max_iter{
        if fupper*fguess<0.0{
            lower=guess;
        }else{
            upper=guess;
        }

        guess=(upper+lower)/2.0;
        fguess=european_call(S,K,r,guess,q,T)-CallPrice;
        iter+=1;
    }
    if iter>max_iter{
        return Err("Failed to converge within maxminum iterration.".to_string());
    }
    Ok(guess)
}

#[cfg(test)]
mod tests {
    use super::*;
    // 引入 assert-approx-eq 宏
    use assert_approx_eq::assert_approx_eq;

    // 测试1：欧式看涨期权经典案例
    #[test]
    fn test_european_call_classic_case() {
        // 经典场景：标的50.0，行权价40，无风险利率5%，波动率30%，无股息2%，到期2年
        let price = european_call(50.0, 40.0, 0.05, 0.3, 0.02, 2.0);
        // 用库的宏验证（精度 1e-4，匹配理论值 10.48306）
        //println!("price: {}", price);
        assert_approx_eq!(price, 14.48306, 1e-4);
    }

    // 测试2: 欧式看涨期权边界条件：T=0（内在价值）
    #[test]
    fn test_european_call_t_zero() {
        let price = european_call(100.0, 90.0, 0.05, 0.2, 0.0, 0.0);
        assert_approx_eq!(price, 10.0, 1e-6); // 100-90=10，精度更高
    }

    // 测试3：欧式看涨期权特殊情景，波动率=0（无波动，价值=折现内在价值）
    #[test]
    fn test_european_call_sigma_zero() {
        let price:f64 = european_call(100.0, 100.0, 0.05, 0.0, 0.0, 1.0);
        let expected:f64 = 100.0 - 100.0 * (-0.05_f64 * 1.0_f64).exp(); // 折现内在价值
        assert_approx_eq!(price, expected, 1e-6);
    }
}