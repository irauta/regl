
use std::rc::Rc;
use std::fmt::Debug;
use std::mem::size_of;
use ::gl::types::{GLenum,GLsizeiptr,GLintptr,GLvoid};
use ::id::{Id,GenerateId};
use ::ReglResult;
use ::ReglError;
use ::GlId;
use ::tracker::BindIf;
use ::resource::ResourceCreationSupport;
use ::vertex_array::{VertexArray,bind_vertex_array};

pub trait BufferCreationSupport : ResourceCreationSupport {
    fn get_default_vertex_array(&mut self) -> Rc<VertexArray>;
}

pub trait BufferSupport : BindIf<VertexBufferTag> + BindIf<IndexBufferTag> + BindIf<UniformBufferTag> + Debug {}

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

#[derive(Debug,Clone,Copy)]
pub enum BufferUsage {
    StreamDraw,
    StreamRead,
    StreamCopy,
    StaticDraw,
    StaticRead,
    StaticCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy,
}

#[derive(Debug)]
pub struct BaseBuffer {
    shared_context: Rc<BufferSupport>,
    uid: Id,
    gl_id: GlId,
    target: BufferTarget,
    usage: BufferUsage,
    default_vertex_array: Rc<VertexArray>,
    data_len: usize,
}

impl BaseBuffer {
    pub fn get_id(&self) -> &Id {
        &self.uid
    }

    pub fn bind_target(&self, target: BufferTarget) {
        match target {
            BufferTarget::VertexBuffer => BindIf::<VertexBufferTag>::bind_if(&*self.shared_context, &self.uid, &|| self.gl_bind(target)),
            BufferTarget::IndexBuffer => BindIf::<IndexBufferTag>::bind_if(&*self.shared_context, &self.uid, &|| self.gl_bind(target)),
            BufferTarget::UniformBuffer => BindIf::<UniformBufferTag>::bind_if(&*self.shared_context, &self.uid, &|| self.gl_bind(target)),
        }
    }

    /// Forces the bind to happen; used to bind IBO to VAO
    pub fn bind_as_indices_anyway(&self) {
        BindIf::<IndexBufferTag>::bind_if(&*self.shared_context, &self.uid, &|| ());
        self.gl_bind(BufferTarget::IndexBuffer);
    }

    pub fn update_data<T>(&self, byte_offset: usize, data: &[T]) -> ReglResult<()> {
        let data_len = len_in_bytes(data);
        let data_end = data_len as usize + byte_offset;
        if data_end > self.data_len as usize {
            return Err(ReglError::BufferDataOutOfRange);
        }
        self.bind_default();
        glcall!(BufferSubData(gl_target(self.target), byte_offset as GLintptr, data_len, data.as_ptr() as *const GLvoid));
        Ok(())
    }

    fn bind_with_default_vao(&self) {
        bind_vertex_array(&*self.default_vertex_array);
        self.bind_target(BufferTarget::IndexBuffer);
    }

    fn bind_default(&self) {
        if let BufferTarget::IndexBuffer = self.target {
            self.bind_with_default_vao();
        } else {
            self.bind_target(self.target);
        }
    }

    fn initial_data<T>(&self, data: &[T]) {
        let data_len = len_in_bytes(data);
        assert_eq!(self.data_len, data_len as usize);
        self.bind_default();
        glcall!(BufferData(gl_target(self.target), data_len, data.as_ptr() as *const GLvoid, gl_usage(self.usage)))
    }

    fn gl_bind(&self, target: BufferTarget) {
        glcall!(BindBuffer(gl_target(target), self.gl_id));
    }
}

impl Drop for BaseBuffer {
    fn drop(&mut self) {
        glcall!(DeleteBuffers(1, &self.gl_id));
    }
}

#[derive(Debug)]
pub struct Buffer {
    base_buffer: Rc<BaseBuffer>
}

impl Buffer {
    pub fn new<C: BufferCreationSupport, T>(support: &mut C, target: BufferTarget, usage: BufferUsage, data: &[T]) -> ReglResult<Buffer> {
        let mut gl_id = 0;
        glcall!(GenBuffers(1, &mut gl_id));
        let base_buffer = BaseBuffer {
            shared_context: support.get_shared_context(),
            uid: support.generate_id(),
            gl_id: gl_id,
            target: target,
            usage: usage,
            default_vertex_array: support.get_default_vertex_array(),
            data_len: len_in_bytes(data) as usize,
        };

        base_buffer.initial_data(data);

        Ok(Buffer { base_buffer: Rc::new(base_buffer) })
    }

    pub fn update_data<T>(&self, byte_offset: usize, data: &[T]) -> ReglResult<()> {
        self.base_buffer.update_data(byte_offset, data)
    }
}

pub fn get_base_buffer(buffer: &Buffer) -> &Rc<BaseBuffer> {
    &buffer.base_buffer
}

impl UpdateBuffer for Buffer {

}

fn len_in_bytes<T>(data: &[T]) -> GLsizeiptr {
    (size_of::<T>() * data.len()) as GLsizeiptr
}

fn gl_target(target: BufferTarget) -> GLenum {
    match target {
        BufferTarget::VertexBuffer => ::gl::ARRAY_BUFFER,
        BufferTarget::IndexBuffer => ::gl::ELEMENT_ARRAY_BUFFER,
        BufferTarget::UniformBuffer => ::gl::UNIFORM_BUFFER,
    }
}

fn gl_usage(usage: BufferUsage) -> GLenum {
    match usage {
        BufferUsage::StreamDraw => ::gl::STREAM_DRAW,
        BufferUsage::StreamRead => ::gl::STREAM_READ,
        BufferUsage::StreamCopy => ::gl::STREAM_COPY,
        BufferUsage::StaticDraw => ::gl::STATIC_DRAW,
        BufferUsage::StaticRead => ::gl::STATIC_READ,
        BufferUsage::StaticCopy => ::gl::STATIC_COPY,
        BufferUsage::DynamicDraw => ::gl::DYNAMIC_DRAW,
        BufferUsage::DynamicRead => ::gl::DYNAMIC_READ,
        BufferUsage::DynamicCopy => ::gl::DYNAMIC_COPY,
    }
}
