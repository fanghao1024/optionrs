use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use crate::errors::OptionError;
use crate::params::common::CommonParams;
use crate::traits::payoff::{AnalyticPayoffType, Payoff};
use crate::traits::engine::{AnalyticCalculator, AnalyticCalculatorRef, PriceEngine};
use crate::traits::exercise::ExerciseRule;
use super::calculators::{VanillaCalculator, BinaryCalculator, BarrierCalculator};
use crate::errors::*;
#[derive(Debug,Clone)]
pub struct AnalyticEngine{
    /// 解析解的计算器注册表:
    /// - key: option type
    /// - value: corresponding calculator plugin
    calculators: HashMap<AnalyticPayoffType,AnalyticCalculatorRef>
}

impl AnalyticEngine {
    pub fn new() -> AnalyticEngine {
        let mut calculators= HashMap::new();
        // register vanilla calculator
        let vanilla_calc=Arc::new(VanillaCalculator) as AnalyticCalculatorRef;
        for typ in vanilla_calc.supported_types() {
            calculators.insert(typ,vanilla_calc.clone());
        }
        // register binary calculator
        let binary_calc=Arc::new(BinaryCalculator) as AnalyticCalculatorRef;
        for typ in binary_calc.supported_types() {
            calculators.insert(typ,binary_calc.clone());
        }
        // register barrier calculator
        let barrier_calc=Arc::new(BarrierCalculator) as Arc<BarrierCalculator>;
        for typ in barrier_calc.supported_types() {
            calculators.insert(typ,barrier_calc.clone());
        }
        Self{calculators}
    }

    /// 动态注册新的解析解计算器（插件化核心：热扩展）
    pub fn register_calculator(&mut self,calculator:AnalyticCalculatorRef){
        for typ in calculator.supported_types() {
            self.calculators.insert(typ,calculator.clone());
        }
    }

    /// 移除指定类型的计算器
    pub fn remove_calculator(&mut self,typ:AnalyticPayoffType){
        self.calculators.remove(&typ);
    }

    /// 获取指定类型的计算器
    pub fn get_calculator(&self,typ:AnalyticPayoffType)->Option<AnalyticCalculatorRef>{
        self.calculators.get(&typ).cloned()
    }
}

impl PriceEngine for AnalyticEngine {
    fn price(&self, params: &CommonParams, payoff: &dyn Payoff, exercise_rule: &dyn ExerciseRule) -> Result<f64> {
        // 解析解只支持欧式期权
        if !exercise_rule.is_european(){
            return Err(
                OptionError::InvalidParameter(
                    "The pricing of analytical solutions only support European rules".into()
                )
            );
        }

        // 获取当前payoff的解析解的类型
        let analytic_type=payoff.analytic_type()
            .ok_or_else(
                 || OptionError::InvalidParameter(
                        format!(
                            "Analytical solution do not \
                            support this type of option ({:?})",payoff.as_any().type_id()
                        )
                    )
            )?;
        let calculator=self.get_calculator(analytic_type)
            .ok_or_else(
                || OptionError::NotImplemented(
                    format!(
                        "Not found {:?} calculator",analytic_type
                    )
                )
            )?;
        calculator.calculate(params,payoff)
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 默认解析解引擎实例
impl Default for AnalyticEngine {
    fn default() -> Self {
        Self::new()
    }
}
