
use std::rc::Rc;

use ::id::{Id,GenerateId};
use ::ReglResult;
use ::GlId;
use ::tracker::BindIf;
use ::resource::ResourceCreationSupport;
use ::vertex_array::{VertexArray,bind_vertex_array};

pub trait BufferCreationSupport : ResourceCreationSupport {
    fn get_default_vertex_array(&mut self) -> Rc<VertexArray>;
}

pub trait BufferSupport : BindIf<VertexBufferTag> + BindIf<IndexBufferTag> + BindIf<UniformBufferTag> {}

pub trait UpdateBuffer {

}

#[allow(dead_code)]
pub struct VertexBufferTag;
#[allow(dead_code)]
pub struct IndexBufferTag;
#[allow(dead_code)]
pub struct UniformBufferTag;

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
    default_vertex_array: Rc<VertexArray>,
}

impl BaseBuffer {
    pub fn get_id(&self) -> &Id {
        &self.uid
    }

    fn bind_with_default_vao(&self) {
        bind_vertex_array(&*self.default_vertex_array);
        self.bind_target(self.target);
    }

    pub fn bind_target(&self, target: BufferTarget) {
        match target {
            BufferTarget::VertexBuffer => BindIf::<VertexBufferTag>::bind_if(&*self.shared_context, &self.uid, &|| self.gl_bind(target)),
            BufferTarget::IndexBuffer => BindIf::<IndexBufferTag>::bind_if(&*self.shared_context, &self.uid, &|| self.gl_bind(target)),
            BufferTarget::UniformBuffer => BindIf::<UniformBufferTag>::bind_if(&*self.shared_context, &self.uid, &|| self.gl_bind(target)),
        }
    }

    fn gl_bind(&self, target: BufferTarget) {
        glcall!(BindBuffer(match target {
            BufferTarget::VertexBuffer => ARRAY_BUFFER,
            BufferTarget::IndexBuffer => ELEMENT_ARRAY_BUFFER,
            BufferTarget::UniformBuffer => UNIFORM_BUFFER,
        }, self.gl_id));
    }
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
    pub fn new(support: &mut BufferCreationSupport, target: BufferTarget) -> ReglResult<Buffer> {
        let base_buffer = BaseBuffer {
            shared_context: support.get_shared_context(),
            uid: support.generate_id(),
            gl_id: 0,
            target: target,
            default_vertex_array: support.get_default_vertex_array(),
        };
        base_buffer.bind_with_default_vao();
        // TODO: Fill in buffer data.
        Ok(Buffer { base_buffer: Rc::new(base_buffer) })
    }
}

pub fn get_base_buffer(buffer: &Buffer) -> &Rc<BaseBuffer> {
    &buffer.base_buffer
}

impl UpdateBuffer for Buffer {

}
