
use std::rc::Rc;
use std::fmt::Debug;
use ::gl::types::{GLenum,GLint};
use ::id::{Id,GenerateId};
use ::ReglResult;
use ::ReglError;
use ::GlId;
use ::tracker::BindIf;
use ::resource::ResourceCreationSupport;
use ::shader::{Shader,InternalShader};

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
    pub fn new<C: ProgramCreationSupport>(support: &mut C, shaders: &[Shader]) -> ReglResult<Program> {
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
