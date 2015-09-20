
use std::ffi::NulError;

#[derive(Debug)]
pub enum ReglError {
    NulError(NulError),
    ShaderCompilationError(String),
    ProgramLinkingError(String),
}

impl From<NulError> for ReglError {
    fn from(error: NulError) -> ReglError {
        ReglError::NulError(error)
    }
}
