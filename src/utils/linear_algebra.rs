use crate::errors::*;

/// Thomas算法求解器（求解三对角线性方程组）
pub fn ThomasSolver(
    a:&[f64],   // 下对角线(长度n-1)
    b:&[f64],   // 主对角线(长度n)
    c:&[f64],   // 上对角线(长度n-1)
    d:&[f64],   // 右端项(长度n)
)->Result<Vec<f64>>{
    let n=d.len();
    if n==0{
        return Ok(vec![]);
    }
    if b.len()!=n || a.len()!=n-1 ||c.len()!=n-1 {
        return Err(OptionError::InvalidParameter("Thamos algorithm: \n \
        the input dim of array not match".to_string()));
    }
    if n==1{
        if b[0].abs()<1e-12{
            return Err(OptionError::CalculationError("Matrix is singular".to_string()));
        }
        return Ok(vec![d[0]/b[0]]);
    }
    // 前向消元
    let mut c_prime=vec![0.0;n];
    let mut d_prime=vec![0.0;n];

    if b[0].abs()<1e-12{
        return Err(OptionError::CalculationError("Matrix is singular".to_string()));
    }
    c_prime[0]=c[0]/b[0];
    d_prime[0]=d[0]/b[0];

    for i in 1..n{
        let denominator=b[i]-a[i-1]*c_prime[i-1];
        if denominator.abs()<1e-12{
            return Err(OptionError::CalculationError(format!("Thomas algorithm calculate failure:\n \
            Principal element is zero in line {}",i)));
        }
        c_prime[i]=if i<n-1{c[i]/denominator}else{0.0};
        d_prime[i]=(d[i]-a[i-1]*d_prime[i-1])/denominator;
    }
    let mut x=vec![0.0;n];
    x[n-1]=d_prime[n-1];
    for i in (0..n-1).rev(){
        x[i]=d_prime[i]-c_prime[i]*x[i+1];
    }
    Ok(x)

}