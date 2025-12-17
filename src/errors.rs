use thiserror::Error;

#[derive(Error, Debug)]
pub enum OptionError{
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Convergence error: {0}")]
    ConvergenceError(String),

    #[error("Calculation error: {0}")]
    CalculationError(String),

    #[error("Arbitration Violation: {0}")]
    ArbitrationViolation(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Data is empty")]
    EmptyData,

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, OptionError>;