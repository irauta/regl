
extern crate gl;

#[macro_use]
mod macros;

mod id;
mod tracker;
mod resource;
mod context;
mod buffer;
mod framebuffer;
mod vertex_array;

type GlId = gl::types::GLuint;

pub type ReglError = ();
pub type ReglResult<T> = Result<T, ReglError>;


pub use gl::load_with;

pub use context::Context;
pub use buffer::{Buffer,BufferTarget};
pub use framebuffer::Framebuffer;
pub use vertex_array::{VertexArray};

pub fn nothing() {}

#[test]
fn it_works() {
}
