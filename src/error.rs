
use std::ffi::NulError;

#[derive(Debug)]
pub enum ReglError {
    NulError(NulError),
    ShaderCompilationError(String),
}

impl From<NulError> for ReglError {
    fn from(error: NulError) -> ReglError {
        ReglError::NulError(error)
    }
}
