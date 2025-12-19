pub mod explicit;
pub mod implicit;
pub mod crank_nicolson;

pub use explicit::ExplicitMethod;
pub use implicit::ImplicitMethod;
pub use crank_nicolson::CrankNicolsonMethod;
/// PDE方法类型枚举
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum FiniteDifferenceMethod{
    Explicit,
    Implicit,
    CrankNicolson,
}