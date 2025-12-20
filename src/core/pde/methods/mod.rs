pub mod explicit;
pub mod implicit;
pub mod crank_nicolson;

pub use explicit::ExplicitMethod;
pub use implicit::ImplicitMethod;
pub use crank_nicolson::CrankNicolsonMethod;

