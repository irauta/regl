
use std::error::Error;
use std::fmt::{self,Display};
use std::ffi::NulError;

#[derive(Debug)]
pub enum ReglError {
    NulError(NulError),
    ShaderCompilationError(String),
    ProgramLinkingError(String),
    BufferDataOutOfRange,
}

impl From<NulError> for ReglError {
    fn from(error: NulError) -> ReglError {
        ReglError::NulError(error)
    }
}

impl Display for ReglError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match additional_message(self) {
            Some(additional) => write!(f, "ReglError: {}; {}", self.description(), additional),
            None => write!(f, "ReglError: {}", self.description()),
        }
    }
}

impl Error for ReglError {
    fn description(&self) -> &str {
        match *self {
            ReglError::NulError(_) => "Null byte encountered in unexpected place",
            ReglError::ShaderCompilationError(_) => "GLSL shader compilation failed",
            ReglError::ProgramLinkingError(_) => "GLSL shader program linking failed",
            ReglError::BufferDataOutOfRange => "Tried to update buffer data beyond buffer end",
        }
    }
}

fn additional_message(error: &ReglError) -> Option<&str> {
    match *error {
        ReglError::NulError(ref error) => Some(error.description()),
        ReglError::ShaderCompilationError(ref msg) => Some(msg.as_ref()),
        ReglError::ProgramLinkingError(ref msg) => Some(msg.as_ref()),
        ReglError::BufferDataOutOfRange => None,
    }
}
