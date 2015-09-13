
use std::rc::Rc;

use ::id::{Id,GenerateId};
use ::ReglResult;
use ::GlId;
use ::resource::ResourceCreationSupport;

pub trait BufferSupport {}

pub trait UpdateBuffer {

}

#[derive(Debug,Clone,Copy)]
pub enum BufferTarget {
    VertexBuffer,
    IndexBuffer,
    UniformBuffer,
}

pub struct BaseBuffer {
    shared_context: Rc<BufferSupport>,
    uid: Id,
    gl_id: GlId,
    target: BufferTarget,
}

impl Drop for BaseBuffer {
    fn drop(&mut self) {
        glcall!(DeleteBuffers(1, &self.gl_id));
    }
}

pub struct Buffer {
    base_buffer: Rc<BaseBuffer>
}

impl Buffer {
    pub fn new(support: &mut ResourceCreationSupport) -> ReglResult<Buffer> {
        let base_buffer = BaseBuffer {
            shared_context: support.get_shared_context(),
            uid: support.generate_id(),
            gl_id: 0,
            target: BufferTarget::VertexBuffer,
        };
        Ok(Buffer { base_buffer: Rc::new(base_buffer) })
    }
}

impl UpdateBuffer for Buffer {

}