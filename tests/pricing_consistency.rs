//! 跨模型集成测试
use optionrs::{black_scholes, binomial};
// 引入 assert-approx-eq 宏
use assert_approx_eq::assert_approx_eq;

#[test]
fn test_bs_vs_binomial_convergence() {
    let s = 100.0;
    let k = 100.0;
    let r = 0.05;
    let sigma = 0.2;
    let q = 0.0;
    let t = 1.0;

    let bs_price = black_scholes::european_call(s, k, r, sigma, q, t);
    // 1000步二叉树，逼近BS结果
    let binomial_price = binomial::european_call_binomial(s, k, r, sigma, q, t, 1000);

    // 验证两者近似相等（精度 0.01，允许二叉树的微小误差）
    assert_approx_eq!(bs_price, binomial_price, 0.01);
}