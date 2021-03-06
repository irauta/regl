
use std::rc::Rc;
use std::fmt::Debug;
use gl::types::{GLenum, GLint};
use id::{Id, GenerateId, GlId};
use ReglResult;
use ReglError;
use tracker::BindIf;
use resource::ResourceCreationSupport;
use shader::{Shader, InternalShader};

pub use self::attribute::{AttributeInfo, ShaderAttribute, ShaderAttributeType};
pub use self::uniform::{UniformInfo, Uniform, InterfaceBlock, BlockUniform, UniformType};

mod attribute;
mod uniform;

pub trait ProgramCreationSupport : ResourceCreationSupport {
    fn validate_after_linking(&self) -> bool;
}

pub trait ProgramSupport : BindIf<Program> + Debug {}

pub trait ProgramInternal {
    fn bind(&self);
}

#[derive(Debug)]
pub struct Program {
    shared_context: Rc<ProgramSupport>,
    uid: Id,
    gl_id: GlId,
}

impl Program {
    pub fn new<C: ProgramCreationSupport>(support: &mut C,
                                          shaders: &[Shader])
                                          -> ReglResult<Program> {
        let gl_id = glcall!(CreateProgram());

        for shader in shaders {
            glcall!(AttachShader(gl_id, shader.gl_id()));
        }

        glcall!(LinkProgram(gl_id));
        if support.validate_after_linking() && !linked(gl_id) {
            return Err(ReglError::ProgramLinkingError(info_log(gl_id)));
        }

        Ok(Program {
            shared_context: support.get_shared_context(),
            uid: support.generate_id(),
            gl_id: gl_id,
        })
    }

    pub fn validate(&self) -> bool {
        gl_program_value(self.gl_id, ::gl::VALIDATE_STATUS) != 0
    }

    pub fn info_log(&self) -> String {
        info_log(self.gl_id)
    }

    pub fn attribute_info(&self) -> AttributeInfo {
        attribute::get_attribute_info(self.gl_id)
    }

    pub fn attribute_location<T: AsRef<str>>(&self, name: T) -> ReglResult<i32> {
        attribute::get_attribute_location(self.gl_id, name.as_ref())
    }

    pub fn uniform_info(&self) -> UniformInfo {
        uniform::get_uniform_info(self.gl_id)
    }

    pub fn uniform_location<T: AsRef<str>>(&self, name: T) -> ReglResult<i32> {
        uniform::get_uniform_location(self.gl_id, name.as_ref())
    }

    pub fn uniform_f32(&self,
                       location: i32,
                       uniform_type: UniformType,
                       count: u32,
                       values: &[f32])
                       -> ReglResult<()> {
        self.bind();
        uniform::uniform_value_f32(location, uniform_type, count, values)
    }

    pub fn uniform_u32(&self,
                       location: i32,
                       uniform_type: UniformType,
                       count: u32,
                       values: &[u32])
                       -> ReglResult<()> {
        self.bind();
        uniform::uniform_value_u32(location, uniform_type, count, values)
    }

    pub fn uniform_i32(&self,
                       location: i32,
                       uniform_type: UniformType,
                       count: u32,
                       values: &[i32])
                       -> ReglResult<()> {
        self.bind();
        uniform::uniform_value_i32(location, uniform_type, count, values)
    }

    pub fn uniform_matrix(&self,
                          location: i32,
                          uniform_type: UniformType,
                          count: u32,
                          values: &[f32],
                          transpose: bool)
                          -> ReglResult<()> {
        self.bind();
        uniform::uniform_value_matrix(location, uniform_type, count, values, transpose)
    }

    fn gl_bind(&self) {
        glcall!(UseProgram(self.gl_id));
    }
}

impl ProgramInternal for Program {
    fn bind(&self) {
        self.shared_context.bind_if(&self.uid, &|| self.gl_bind());
    }
}

fn linked(gl_id: GlId) -> bool {
    gl_program_value(gl_id, ::gl::LINK_STATUS) != 0
}

fn info_log_length(gl_id: GlId) -> GLint {
    gl_program_value(gl_id, ::gl::INFO_LOG_LENGTH)
}

fn gl_program_value(gl_id: GlId, key: GLenum) -> GLint {
    let mut value = 0;
    glcall!(GetProgramiv(gl_id, key, &mut value));
    value
}

fn info_log(gl_id: GlId) -> String {
    let len = info_log_length(gl_id);
    let mut actual_len = 0;
    let mut log = vec![0u8; len as usize];
    // len - 1 because info_log_length returns the value *including* null terminator
    glcall!(GetProgramInfoLog(gl_id, len - 1, &mut actual_len, log.as_mut_ptr() as *mut i8));
    log.truncate(actual_len as usize);
    String::from_utf8_lossy(&log[..]).into_owned()
}

impl Drop for Program {
    fn drop(&mut self) {
        glcall!(DeleteProgram(self.gl_id));
    }
}
