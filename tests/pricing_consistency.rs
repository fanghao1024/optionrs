//! 跨模型集成测试
/*
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


 */

use optionrs::*;
use std::sync::Arc;
#[test]
fn test_vanilla_option_analytic() {
    // 1. 初始化参数：欧式看涨期权（S=100, K=100, r=5%, σ=20%, t=1年, q=0）
    let params = CommonParams::new(100.0, 0.05, 0.2, 0.0, 1.0).unwrap();
    let payoff = optionrs::traits::payoff::CallPayoff { strike: 100.0 };
    let exercise = EuropeanExercise;

    // 2. 创建解析解引擎
    let engine = EngineConfig::default_analytic().unwrap();

    // 3. 计算价格（理论值≈10.4506）
    let price = engine.calculate_price(&params, &payoff, &exercise).unwrap();
    assert!((price - 10.4506).abs() < 1e-4, "普通看涨期权价格计算错误：{}", price);
}

#[test]
fn test_binary_option_analytic() {
    // 1. 初始化参数：现金或无看涨二元期权（S=100, K=100, 赔付10元, r=5%, σ=20%, t=1年）
    let params = CommonParams::new(100.0, 0.05, 0.2, 0.0, 1.0).unwrap();
    let payoff = optionrs::traits::payoff::CashOrNothingCallPayoff { strike: 100.0, payout: 10.0 };
    let exercise = EuropeanExercise;

    // 2. 计算价格（理论值≈5.82）
    let engine = EngineConfig::default_analytic().unwrap();
    let price = engine.calculate_price(&params, &payoff, &exercise).unwrap();
    assert!((price - 5.82).abs() < 1e-2, "二元期权价格计算错误：{}", price);
}

#[test]
fn test_barrier_option_analytic() {
    // 1. 初始化参数：向下敲出看涨障碍期权（S=100, K=100, 障碍价=80, r=5%, σ=20%, t=1年）
    let params = CommonParams::new(100.0, 0.05, 0.2, 0.0, 1.0).unwrap();
    let payoff = optionrs::traits::payoff::DownAndOutCallPayoff { strike: 100.0, barrier: 80.0 };
    let exercise = EuropeanExercise;

    // 2. 计算价格（理论值≈9.2）
    let engine = EngineConfig::default_analytic().unwrap();
    let price = engine.calculate_price(&params, &payoff, &exercise).unwrap();
    assert!((price - 9.2).abs() < 1e-1, "障碍期权价格计算错误：{}", price);
}

#[test]
fn test_dynamic_register_calculator() {
    // 1. 创建空解析解引擎（仅演示动态注册）
    let mut analytic_engine = AnalyticEngine::new();
    // 2. 验证初始状态：已注册普通期权计算器
    assert!(analytic_engine.get_calculator(optionrs::traits::payoff::AnalyticPayoffType::VanillaCall).is_some());
    // 3. 动态移除普通看跌计算器
    analytic_engine.remove_calculator(optionrs::traits::payoff::AnalyticPayoffType::VanillaPut);
    assert!(analytic_engine.get_calculator(optionrs::traits::payoff::AnalyticPayoffType::VanillaPut).is_none());

    // 4. 重新注册普通期权计算器
    let vanilla_calc = Arc::new(optionrs::core::analytic::calculators::VanillaCalculator) as optionrs::traits::engine::AnalyticCalculatorRef;
    analytic_engine.register_calculator(vanilla_calc);
    assert!(analytic_engine.get_calculator(traits::payoff::AnalyticPayoffType::VanillaPut).is_some());
}