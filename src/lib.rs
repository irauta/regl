
extern crate gl;

#[macro_use]
mod macros;

mod error;
mod id;
mod tracker;
mod resource;
mod context;
mod buffer;
mod framebuffer;
mod vertex_array;
mod program;

type GlId = gl::types::GLuint;

pub type ReglResult<T> = Result<T, ReglError>;


pub use gl::load_with;

pub use error::ReglError;
pub use context::Context;
pub use buffer::{Buffer,BufferTarget};
pub use framebuffer::Framebuffer;
pub use vertex_array::{VertexArray,VertexAttributeType,VertexAttribute};
pub use program::Program;
pub use program::{ShaderType,ShaderSource};
