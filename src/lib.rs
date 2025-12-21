//! 金融衍生品定价库，包含多种期权定价模型和模拟工具

// 导出所有公共模块和API

pub mod products;
pub mod core;
pub mod params;
pub mod traits;
pub mod simulation;
pub mod errors;
pub mod utils;

pub mod prelude {
    pub use std::sync::Arc;
    pub use crate::traits::engine::PriceEngine;
    pub use crate::core::engine_config::EngineConfig;
    pub use crate::params::common::CommonParams;
    pub use crate::core::analytic::engine::AnalyticEngine;
    pub use crate::errors::*;
    pub use crate::traits::engine::pricing_trait;
    pub use crate::simulation::brownian::GeometricBrownianMotion;
    pub use crate::core::pde::engine::FiniteDifferenceMethod;
    pub use crate::traits::exercise::{EuropeanExercise,ExerciseRule,AmericanExercise};
}