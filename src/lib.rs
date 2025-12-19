//! 金融衍生品定价库，包含多种期权定价模型和模拟工具

// 导出所有公共模块和API

pub mod products;
pub mod core;
pub mod params;
pub mod traits;
pub mod simulation;
pub mod errors;
pub mod utils;

pub use traits::exercise::{EuropeanExercise,ExerciseRule,AmericanExercise};
pub use traits::engine::PriceEngine;
pub use core::engine_config::EngineConfig;
pub use core::analytic::engine::AnalyticEngine;
pub use params::common::CommonParams;
pub use errors::*;