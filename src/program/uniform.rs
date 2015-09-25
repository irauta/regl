
use ::gl::types::{GLenum,GLint,GLuint,GLsizei};
use super::gl_program_value;

// To see the definition of UniformType, look at the bottom of file. It's the really big enum.

#[derive(Debug)]
pub struct Uniform {
    /// Name of the uniform.
    pub name: String,
    /// Location of the uniform, use this when setting value of the uniform, not the index in the
    /// vector that describes the uniforms; they may not be the same.
    pub location: i32,
    /// Data type of the uniform.
    pub uniform_type: UniformType,
    /// How many instances of the type this uniform contains. Length of an array so to speak.
    pub size: i32
}

fn uniform(gl_uniform: GlUniform) -> Uniform {
    Uniform {
        name: gl_uniform.name,
        location: 0,
        uniform_type: (gl_uniform.uniform_type as GLenum).into(),
        size: gl_uniform.size,
    }
}

fn block_uniform(gl_uniform: GlUniform) -> BlockUniform {
    BlockUniform {
        name: gl_uniform.name,
        uniform_type: (gl_uniform.uniform_type as GLenum).into(),
        size: gl_uniform.size,
        offset: gl_uniform.offset,
        array_stride: gl_uniform.array_stride,
        matrix_stride: gl_uniform.matrix_stride,
    }
}

/// Description of an interface block.
#[derive(Debug)]
pub struct InterfaceBlock {
    /// Name of the block.
    pub name: String,
    /// Index of the block. Use this as the location/index, not the index in the vector this
    /// struct is in!
    pub index: u32,
    /// See GL_UNIFORM_BLOCK_DATA_SIZE
    pub data_size: i32,
    /// The uniforms contained by this block.
    pub uniforms: Vec<BlockUniform>
}

/// A uniform contained within a block.
/// TODO: Missing info whether a matrix uniform is row major.
#[derive(Debug)]
pub struct BlockUniform {
    /// Name of the uniform.
    pub name: String,
    /// Data type of the uniform.
    pub uniform_type: UniformType,
    /// How long is the array of the uniforms (if the uniform is an array uniform).
    pub size: i32,
    /// How many bytes from the beginning of the block this uniform is. See GL_UNIFORM_OFFSET.
    pub offset: i32,
    /// For an array uniform, the distance between each value in the array.
    /// See GL_UNIFORM_ARRAY_STRIDE.
    pub array_stride: i32,
    /// Distance between rows/cols of a matrix uniform. See GL_UNIFORM_MATRIX_STRIDE.
    pub matrix_stride: i32,
}

/// Top-level result structure for program's uniform introspection info.
#[derive(Debug)]
pub struct UniformInfo {
    /// Global uniforms, not in interface blocks.
    pub globals: Vec<Uniform>,
    /// Interface block definitions, may contain several uniforms themselves.
    pub blocks: Vec<InterfaceBlock>
}

#[derive(Default)]
struct GlUniform {
    name: String,
    uniform_type: i32,
    size: i32,
    block_index: i32,
    offset: i32,
    array_stride: i32,
    matrix_stride: i32,
}

pub fn get_uniform_info(program_id: GLuint) -> UniformInfo {
    let gl_uniforms = get_gl_uniforms(program_id);
    let mut globals = vec![];
    let mut blocks = get_uniform_blocks(program_id);
    for gl_uniform in gl_uniforms.into_iter() {
        if gl_uniform.block_index < 0 {
            //let location = program.get_uniform_location(&gl_uniform.name[..]);
            globals.push(uniform(gl_uniform));
        }
        else {
            let index = gl_uniform.block_index as usize;
            blocks[index].uniforms.push(block_uniform(gl_uniform));
        }
    }
    UniformInfo {
        globals: globals,
        blocks: blocks,
    }
}

fn get_gl_uniforms(program_id: GLuint) -> Vec<GlUniform> {
    let count = gl_program_value(program_id, ::gl::ACTIVE_UNIFORMS) as usize;
    if count == 0 {
        return vec![];
    }
    let indices: Vec<GLuint> = (0..count as GLuint).collect();
    let mut intvalues = vec![0; count];
    gl_uniform_properties(program_id, &indices, ::gl::UNIFORM_NAME_LENGTH, &mut intvalues);

    let mut uniforms: Vec<GlUniform> = intvalues.iter().enumerate().map(
        |(index, name_length)| GlUniform {
            name: gl_uniform_name(program_id, index as GLuint, *name_length),
            .. Default::default()
        }
    ).collect();
    // Can't return uniforms before the closure using them is out of scope
    {
        let mut fill_info = |gl_property, uniform_field_fn: &Fn(&mut GlUniform) -> &mut GLint| {
            gl_uniform_properties(program_id, &indices, gl_property, &mut intvalues);
            for (gl_uniform, value) in uniforms.iter_mut().zip(intvalues.iter()) {
                *uniform_field_fn(gl_uniform) = *value;
            }
        };
        fill_info(::gl::UNIFORM_SIZE, &|u| &mut u.size);
        fill_info(::gl::UNIFORM_TYPE, &|u| &mut u.uniform_type);
        fill_info(::gl::UNIFORM_OFFSET, &|u| &mut u.offset);
        fill_info(::gl::UNIFORM_BLOCK_INDEX, &|u| &mut u.block_index);
        fill_info(::gl::UNIFORM_ARRAY_STRIDE, &|u| &mut u.array_stride);
        fill_info(::gl::UNIFORM_MATRIX_STRIDE, &|u| &mut u.matrix_stride);
    }
    uniforms
}

fn gl_uniform_properties(program_id: GLuint, indices: &Vec<GLuint>, property: GLenum, intvalues: &mut Vec<GLint>) {
    assert_eq!(indices.len(), intvalues.len());
    glcall!(GetActiveUniformsiv(
        program_id,
        indices.len() as GLsizei,
        indices.as_ptr(),
        property,
        intvalues.as_mut_ptr()
    ));
}

fn gl_uniform_name(program_id: GLuint, index: GLuint, length: GLsizei) -> String {
    let mut name_bytes = vec![0u8; length as usize];
    let name_bytes_ptr = name_bytes.as_mut_ptr() as *mut i8;
    let mut actual_length = 0;
    glcall!(GetActiveUniformName(program_id, index, length, &mut actual_length, name_bytes_ptr));
    String::from_utf8_lossy(&name_bytes[0..actual_length as usize]).into_owned()
}

fn get_uniform_blocks(program_id: GLuint) -> Vec<InterfaceBlock> {
    let count = gl_program_value(program_id, ::gl::ACTIVE_UNIFORM_BLOCKS) as u32;
    (0..count).map(|index| {
        InterfaceBlock {
            index: index,
            name: gl_block_name(program_id, index),
            data_size: gl_block_property(program_id, index, ::gl::UNIFORM_BLOCK_DATA_SIZE),
            uniforms: vec![],
        }
    }).collect()
}

fn gl_block_name(program_id: GLuint, index: GLuint) -> String {
    let length = gl_block_property(program_id, index, ::gl::UNIFORM_BLOCK_NAME_LENGTH);
    let mut name_bytes = vec![0u8; length as usize];
    let name_bytes_ptr = name_bytes.as_mut_ptr() as *mut i8;
    let mut actual_length = 0;
    glcall!(GetActiveUniformBlockName(
        program_id,
        index,
        name_bytes.len() as i32,
        &mut actual_length,
        name_bytes_ptr
    ));
    String::from_utf8_lossy(&name_bytes[0..actual_length as usize]).into_owned()
}

fn gl_block_property(program_id: GLuint, block_index: GLuint, property: GLenum) -> i32 {
    let mut value = 0;
    glcall!(GetActiveUniformBlockiv(program_id, block_index, property, &mut value));
    value
}

#[derive(Debug,Clone,Copy)]
pub enum UniformType {
    Float,
    FloatVec2,
    FloatVec3,
    FloatVec4,
    Int,
    IntVec2,
    IntVec3,
    IntVec4,
    UnsignedInt,
    UnsignedIntVec2,
    UnsignedIntVec3,
    UnsignedIntVec4,
    Bool,
    BoolVec2,
    BoolVec3,
    BoolVec4,
    FloatMat2,
    FloatMat3,
    FloatMat4,
    FloatMat2x3,
    FloatMat2x4,
    FloatMat3x2,
    FloatMat3x4,
    FloatMat4x2,
    FloatMat4x3,
    Sampler1d,
    Sampler2d,
    Sampler3d,
    SamplerCube,
    Sampler1dShadow,
    Sampler2dShadow,
    Sampler1dArray,
    Sampler2dArray,
    Sampler1dArrayShadow,
    Sampler2dArrayShadow,
    Sampler2dMultisample,
    Sampler2dMultisampleArray,
    SamplerCubeShadow,
    SamplerBuffer,
    Sampler2dRect,
    Sampler2dRectShadow,
    IntSampler1d,
    IntSampler2d,
    IntSampler3d,
    IntSamplerCube,
    IntSampler1dArray,
    IntSampler2dArray,
    IntSampler2dMultisample,
    IntSampler2dMultisampleArray,
    IntSamplerBuffer,
    IntSampler2dRect,
    UnsignedIntSampler1d,
    UnsignedIntSampler2d,
    UnsignedIntSampler3d,
    UnsignedIntSamplerCube,
    UnsignedIntSampler1dArray,
    UnsignedIntSampler2dArray,
    UnsignedIntSampler2dMultisample,
    UnsignedIntSampler2dMultisampleArray,
    UnsignedIntSamplerBuffer,
    UnsignedIntSampler2dRect,
    UnrecognizedType(u32)
}

impl From<GLenum> for UniformType {
    fn from(gl_type: GLenum) -> UniformType {
        match gl_type {
            ::gl::FLOAT => UniformType::Float,
            ::gl::FLOAT_VEC2 => UniformType::FloatVec2,
            ::gl::FLOAT_VEC3 => UniformType::FloatVec3,
            ::gl::FLOAT_VEC4 => UniformType::FloatVec4,
            ::gl::INT => UniformType::Int,
            ::gl::INT_VEC2 => UniformType::IntVec2,
            ::gl::INT_VEC3 => UniformType::IntVec3,
            ::gl::INT_VEC4 => UniformType::IntVec4,
            ::gl::UNSIGNED_INT => UniformType::UnsignedInt,
            ::gl::UNSIGNED_INT_VEC2 => UniformType::UnsignedIntVec2,
            ::gl::UNSIGNED_INT_VEC3 => UniformType::UnsignedIntVec3,
            ::gl::UNSIGNED_INT_VEC4 => UniformType::UnsignedIntVec4,
            ::gl::BOOL => UniformType::Bool,
            ::gl::BOOL_VEC2 => UniformType::BoolVec2,
            ::gl::BOOL_VEC3 => UniformType::BoolVec3,
            ::gl::BOOL_VEC4 => UniformType::BoolVec4,
            ::gl::FLOAT_MAT2 => UniformType::FloatMat2,
            ::gl::FLOAT_MAT3 => UniformType::FloatMat3,
            ::gl::FLOAT_MAT4 => UniformType::FloatMat4,
            ::gl::FLOAT_MAT2x3 => UniformType::FloatMat2x3,
            ::gl::FLOAT_MAT2x4 => UniformType::FloatMat2x4,
            ::gl::FLOAT_MAT3x2 => UniformType::FloatMat3x2,
            ::gl::FLOAT_MAT3x4 => UniformType::FloatMat3x4,
            ::gl::FLOAT_MAT4x2 => UniformType::FloatMat4x2,
            ::gl::FLOAT_MAT4x3 => UniformType::FloatMat4x3,
            ::gl::SAMPLER_1D => UniformType::Sampler1d,
            ::gl::SAMPLER_2D => UniformType::Sampler2d,
            ::gl::SAMPLER_3D => UniformType::Sampler3d,
            ::gl::SAMPLER_CUBE => UniformType::SamplerCube,
            ::gl::SAMPLER_1D_SHADOW => UniformType::Sampler1dShadow,
            ::gl::SAMPLER_2D_SHADOW => UniformType::Sampler2dShadow,
            ::gl::SAMPLER_1D_ARRAY => UniformType::Sampler1dArray,
            ::gl::SAMPLER_2D_ARRAY => UniformType::Sampler2dArray,
            ::gl::SAMPLER_1D_ARRAY_SHADOW => UniformType::Sampler1dArrayShadow,
            ::gl::SAMPLER_2D_ARRAY_SHADOW => UniformType::Sampler2dArrayShadow,
            ::gl::SAMPLER_2D_MULTISAMPLE => UniformType::Sampler2dMultisample,
            ::gl::SAMPLER_2D_MULTISAMPLE_ARRAY => UniformType::Sampler2dMultisampleArray,
            ::gl::SAMPLER_CUBE_SHADOW => UniformType::SamplerCubeShadow,
            ::gl::SAMPLER_BUFFER => UniformType::SamplerBuffer,
            ::gl::SAMPLER_2D_RECT => UniformType::Sampler2dRect,
            ::gl::SAMPLER_2D_RECT_SHADOW => UniformType::Sampler2dRectShadow,
            ::gl::INT_SAMPLER_1D => UniformType::IntSampler1d,
            ::gl::INT_SAMPLER_2D => UniformType::IntSampler2d,
            ::gl::INT_SAMPLER_3D => UniformType::IntSampler3d,
            ::gl::INT_SAMPLER_CUBE => UniformType::IntSamplerCube,
            ::gl::INT_SAMPLER_1D_ARRAY => UniformType::IntSampler1dArray,
            ::gl::INT_SAMPLER_2D_ARRAY => UniformType::IntSampler2dArray,
            ::gl::INT_SAMPLER_2D_MULTISAMPLE => UniformType::IntSampler2dMultisample,
            ::gl::INT_SAMPLER_2D_MULTISAMPLE_ARRAY => UniformType::IntSampler2dMultisampleArray,
            ::gl::INT_SAMPLER_BUFFER => UniformType::IntSamplerBuffer,
            ::gl::INT_SAMPLER_2D_RECT => UniformType::IntSampler2dRect,
            ::gl::UNSIGNED_INT_SAMPLER_1D => UniformType::UnsignedIntSampler1d,
            ::gl::UNSIGNED_INT_SAMPLER_2D => UniformType::UnsignedIntSampler2d,
            ::gl::UNSIGNED_INT_SAMPLER_3D => UniformType::UnsignedIntSampler3d,
            ::gl::UNSIGNED_INT_SAMPLER_CUBE => UniformType::UnsignedIntSamplerCube,
            ::gl::UNSIGNED_INT_SAMPLER_1D_ARRAY => UniformType::UnsignedIntSampler1dArray,
            ::gl::UNSIGNED_INT_SAMPLER_2D_ARRAY => UniformType::UnsignedIntSampler2dArray,
            ::gl::UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE => UniformType::UnsignedIntSampler2dMultisample,
            ::gl::UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY => UniformType::UnsignedIntSampler2dMultisampleArray,
            ::gl::UNSIGNED_INT_SAMPLER_BUFFER => UniformType::UnsignedIntSamplerBuffer,
            ::gl::UNSIGNED_INT_SAMPLER_2D_RECT => UniformType::UnsignedIntSampler2dRect,
            other => UniformType::UnrecognizedType(other)
        }
    }
}
