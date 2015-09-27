
extern crate gl;

#[macro_use]
mod macros;

mod error;
mod id;
mod tracker;
mod resource;
mod context;
mod options;
mod buffer;
mod framebuffer;
mod vertex_array;
mod shader;
mod program;

type GlId = gl::types::GLuint;

pub type ReglResult<T> = Result<T, ReglError>;


pub use gl::load_with;

pub use error::ReglError;
pub use context::{Context,PrimitiveMode};
pub use options::RenderOption;
pub use buffer::{Buffer,BufferTarget,BufferUsage};
pub use framebuffer::Framebuffer;
pub use vertex_array::{VertexArray,VertexAttributeType,VertexAttribute};
pub use shader::{Shader,ShaderType,ShaderSource};
pub use program::Program;
pub use program::{AttributeInfo,ShaderAttribute,ShaderAttributeType};
pub use program::{UniformInfo,Uniform,InterfaceBlock,BlockUniform,UniformType};
