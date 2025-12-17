use statrs::distribution::{Normal,Continuous,ContinuousCDF};
use owens_t;

/// Standard normal distribution CDF <br>
/// 标准正态分布的CDF（累积分布函数）
pub fn norm_cdf(x:f64)->f64{
    let normal=Normal::new(0.0, 1.0).expect("Failed to create normal distribution");
    normal.cdf(x)
}

/// Standard normal distribution pdf <br>
/// 标准正态分布的PDF（概率密度函数)
pub fn norm_pdf(x:f64)->f64{
    let normal=Normal::new(0.0, 1.0).expect("Failed to create normal distribution");
    normal.pdf(x)
}

/// CDF of binary normal distribution <br>
/// 二元正态分布的CDF
pub fn bivariate_norm_cdf(a: f64, b: f64, rho: f64) -> f64 {
    use owens_t::biv_norm;

    let phi_a = norm_cdf(a);
    let phi_b = norm_cdf(b);
    let p_gt_a_gt_b = biv_norm(a, b, rho);

    // P(<a, <b) = P(<a) + P(<b) - 1 + P(>a, >b)
    phi_a + phi_b - 1.0 + p_gt_a_gt_b
}

