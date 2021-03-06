
use std::ffi::CString;
use gl::types::{GLuint, GLenum};
use ReglResult;
use super::gl_program_value;

#[derive(Debug,Clone,Copy)]
pub enum ShaderAttributeType {
    Float,
    FloatVec2,
    FloatVec3,
    FloatVec4,
    FloatMat2,
    FloatMat3,
    FloatMat4,
    FloatMat2x3,
    FloatMat2x4,
    FloatMat3x2,
    FloatMat3x4,
    FloatMat4x2,
    FloatMat4x3,
    Int,
    IntVec2,
    IntVec3,
    IntVec4,
    UnsignedInt,
    UnsignedIntVec2,
    UnsignedIntVec3,
    UnsignedIntVec4,
    UnrecognizedType(u32),
}

/// Describes an (active) attribute of a shader program.
#[derive(Debug)]
pub struct ShaderAttribute {
    /// Name of the attribute
    pub name: String,
    /// Index of the attribute
    pub location: i32,
    /// Data type of the attribute
    pub attribute_type: ShaderAttributeType,
    /// Size of the attribute, counted as instances of the shaderattributetype
    pub size: i32,
}

#[derive(Debug)]
pub struct AttributeInfo {
    pub attributes: Vec<ShaderAttribute>,
}

impl AttributeInfo {
    pub fn find<T: AsRef<str>>(&self, name: T) -> Option<&ShaderAttribute> {
        self.attributes.iter().find(|a| a.name == name.as_ref())
    }
}

pub fn get_attribute_info(program_id: GLuint) -> AttributeInfo {
    let attr_count = gl_program_value(program_id, ::gl::ACTIVE_ATTRIBUTES);
    if attr_count == 0 {
        return AttributeInfo { attributes: vec![] };
    }
    let max_length = gl_program_value(program_id, ::gl::ACTIVE_ATTRIBUTE_MAX_LENGTH);
    let mut name_buffer = vec![0u8; max_length as usize];

    let attributes = (0..attr_count)
                         .map(|i| make_attribute(program_id, i, &mut name_buffer))
                         .collect();

    AttributeInfo { attributes: attributes }
}

fn make_attribute(program_id: GLuint, index: i32, name_buffer: &mut [u8]) -> ShaderAttribute {
    let mut actual_length = 0;
    let mut size = 0;
    let mut gl_type = 0;
    let name_buffer_ptr = name_buffer.as_mut_ptr() as *mut i8;

    glcall!(GetActiveAttrib(program_id,
                            index as u32,
                            name_buffer.len() as i32,
                            &mut actual_length,
                            &mut size,
                            &mut gl_type,
                            name_buffer_ptr));
    let location = glcall!(GetAttribLocation(program_id, name_buffer_ptr));

    ShaderAttribute {
        name: String::from_utf8_lossy(&name_buffer[0..actual_length as usize]).into_owned(),
        location: location,
        attribute_type: gl_type.into(),
        size: size,
    }
}

pub fn get_attribute_location(program_id: GLuint, name: &str) -> ReglResult<i32> {
    let c_name = try!(CString::new(name));
    Ok(glcall!(GetAttribLocation(program_id, c_name.as_ptr())))
}

impl From<GLenum> for ShaderAttributeType {
    fn from(gl_type: GLenum) -> ShaderAttributeType {
        match gl_type {
            ::gl::FLOAT => ShaderAttributeType::Float,
            ::gl::FLOAT_VEC2 => ShaderAttributeType::FloatVec2,
            ::gl::FLOAT_VEC3 => ShaderAttributeType::FloatVec3,
            ::gl::FLOAT_VEC4 => ShaderAttributeType::FloatVec4,
            ::gl::FLOAT_MAT2 => ShaderAttributeType::FloatMat2,
            ::gl::FLOAT_MAT3 => ShaderAttributeType::FloatMat3,
            ::gl::FLOAT_MAT4 => ShaderAttributeType::FloatMat4,
            ::gl::FLOAT_MAT2x3 => ShaderAttributeType::FloatMat2x3,
            ::gl::FLOAT_MAT2x4 => ShaderAttributeType::FloatMat2x4,
            ::gl::FLOAT_MAT3x2 => ShaderAttributeType::FloatMat3x2,
            ::gl::FLOAT_MAT3x4 => ShaderAttributeType::FloatMat3x4,
            ::gl::FLOAT_MAT4x2 => ShaderAttributeType::FloatMat4x2,
            ::gl::FLOAT_MAT4x3 => ShaderAttributeType::FloatMat4x3,
            ::gl::INT => ShaderAttributeType::Int,
            ::gl::INT_VEC2 => ShaderAttributeType::IntVec2,
            ::gl::INT_VEC3 => ShaderAttributeType::IntVec3,
            ::gl::INT_VEC4 => ShaderAttributeType::IntVec4,
            ::gl::UNSIGNED_INT => ShaderAttributeType::UnsignedInt,
            ::gl::UNSIGNED_INT_VEC2 => ShaderAttributeType::UnsignedIntVec2,
            ::gl::UNSIGNED_INT_VEC3 => ShaderAttributeType::UnsignedIntVec3,
            ::gl::UNSIGNED_INT_VEC4 => ShaderAttributeType::UnsignedIntVec4,
            other => ShaderAttributeType::UnrecognizedType(other),
        }
    }
}
