//! 金融衍生品定价库，包含多种期权定价模型和模拟工具

// 导出所有公共模块和API

pub mod products;
pub mod core;
pub mod params;
pub mod traits;
pub mod simulation;
pub mod errors;
mod utils;