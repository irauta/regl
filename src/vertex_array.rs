
use std::rc::Rc;
use std::fmt::Debug;
use ::gl::types::{GLenum,GLuint,GLint,GLboolean,GLsizei,GLvoid};
use ::id::{Id,GenerateId};
use ::ReglResult;
use ::GlId;
use ::tracker::BindIf;
use ::resource::ResourceCreationSupport;
use ::buffer::{Buffer,BaseBuffer,BufferTarget,IndexBufferTag,get_base_buffer};

pub trait VertexArraySupport : BindIf<VertexArray> + BindIf<IndexBufferTag> + Debug {
    fn separate_ibo_binding(&self) -> bool;
}

pub trait VertexArrayInternal {
    fn bind(&self);
}

#[derive(Debug)]
pub struct VertexArray {
    shared_context: Rc<VertexArraySupport>,
    uid: Id,
    gl_id: GlId,
    attributes: Vec<StoredVertexAttribute>,
    index_buffer: Option<Rc<BaseBuffer>>,
}

#[derive(Copy,Clone,Debug)]
pub enum VertexAttributeType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    HalfFloat,
    Float,
    Double,
    Int2101010Rev,
    UnsignedInt2101010Rev
}

#[derive(Copy,Clone,Debug)]
pub struct VertexAttribute<'a> {
    pub index: u32,
    pub size: u8,
    pub attribute_type: VertexAttributeType,
    pub normalized: bool,
    pub stride: u32,
    pub offset: u32,
    pub vertex_buffer: &'a Buffer,
}

#[derive(Clone,Debug)]
struct StoredVertexAttribute {
    pub index: u32,
    pub size: u8,
    pub attribute_type: VertexAttributeType,
    pub normalized: bool,
    pub stride: u32,
    pub offset: u32,
    pub vertex_buffer: Rc<BaseBuffer>,
}

impl VertexArray {
    pub fn new<C: ResourceCreationSupport>(support: &mut C, attributes: &[VertexAttribute], index_buffer: Option<&Buffer>) -> ReglResult<VertexArray> {
        let mut gl_id = 0;
        glcall!(GenVertexArrays(1, &mut gl_id));
        let vertex_array = VertexArray {
            shared_context: support.get_shared_context(),
            uid: support.generate_id(),
            gl_id: gl_id,
            attributes: attributes.iter().map(into_stored).collect(),
            index_buffer: index_buffer.map(|b| get_base_buffer(b).clone()),
        };
        vertex_array.bind();
        setup_vertex_array(&*vertex_array.shared_context, &vertex_array.attributes[..], index_buffer.map(get_base_buffer));
        Ok(vertex_array)
    }

    fn gl_bind(&self) {
        glcall!(BindVertexArray(self.gl_id));
    }
}

impl VertexArrayInternal for VertexArray {
    fn bind(&self) {
        //self.shared_context.bind_if(&self.uid, &|| self.gl_bind());
        BindIf::<VertexArray>::bind_if(&*self.shared_context, &self.uid, &|| self.gl_bind());
        // TODO: Handle the cases where IBO binding isn't part of VAO state!
        match (&self.index_buffer, self.shared_context.separate_ibo_binding()) {
            (&Some(ref ibo), true) => ibo.bind_target(BufferTarget::IndexBuffer),
            _ => {}
        }
    }
}

/// Expects that the vertex array has already been bound
fn setup_vertex_array(shared_context: &VertexArraySupport, attributes: &[StoredVertexAttribute], index_buffer: Option<&Rc<BaseBuffer>>) {
    if let Some(ref ibo) = index_buffer {
        BindIf::<IndexBufferTag>::bind_if(shared_context, &ibo.get_id(), &|| ibo.bind_target(BufferTarget::IndexBuffer));
    }
    for attribute in attributes {
        attribute.vertex_buffer.bind_target(BufferTarget::VertexBuffer);
        glcall!(EnableVertexAttribArray(attribute.index));
        glcall!(VertexAttribPointer(
            attribute.index as GLuint,
            attribute.size as GLint,
            attribute_to_gl_type(attribute.attribute_type),
            attribute.normalized as GLboolean,
            attribute.stride as GLsizei,
            attribute.offset as *const GLvoid
        ));
    }
}

pub fn bind_vertex_array(vertex_array: &VertexArray) {
    vertex_array.bind();
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        glcall!(DeleteVertexArrays(1, &self.gl_id));
    }
}

pub fn create_default_vertex_array<C: ResourceCreationSupport>(support: &mut C) -> ReglResult<VertexArray> {
    VertexArray::new(support, &[], None)
}

fn into_stored<'a>(attribute: &VertexAttribute<'a>) -> StoredVertexAttribute {
    StoredVertexAttribute {
        index: attribute.index,
        size: attribute.size,
        attribute_type: attribute.attribute_type,
        normalized: attribute.normalized,
        stride: attribute.stride,
        offset: attribute.offset,
        vertex_buffer: get_base_buffer(attribute.vertex_buffer).clone(),
    }
}

fn attribute_to_gl_type(attribute_type: VertexAttributeType) -> GLenum {
    match attribute_type {
        VertexAttributeType::Byte => ::gl::BYTE,
        VertexAttributeType::UnsignedByte => ::gl::UNSIGNED_BYTE,
        VertexAttributeType::Short => ::gl::SHORT,
        VertexAttributeType::UnsignedShort => ::gl::UNSIGNED_SHORT,
        VertexAttributeType::Int => ::gl::INT,
        VertexAttributeType::UnsignedInt => ::gl::UNSIGNED_INT,
        VertexAttributeType::HalfFloat => ::gl::HALF_FLOAT,
        VertexAttributeType::Float => ::gl::FLOAT,
        VertexAttributeType::Double => ::gl::DOUBLE,
        VertexAttributeType::Int2101010Rev => ::gl::INT_2_10_10_10_REV,
        VertexAttributeType::UnsignedInt2101010Rev => ::gl::UNSIGNED_INT_2_10_10_10_REV
    }
}
