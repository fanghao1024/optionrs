use std::collections::HashMap;
use std::sync::Arc;


#[derive(Debug,Clone)]
pub struct AnalyticEngine{
    /// 解析解的计算器注册表:
    /// - key: option type
    /// - value: corresponding calculator plugin
    calculator: HashMap<>
}