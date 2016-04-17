
use std::ffi::CString;
use gl::types::{GLenum, GLint};
use id::GlId;
use resource::ResourceCreationSupport;
use ReglResult;
use ReglError;

pub trait ShaderCreationSupport : ResourceCreationSupport {
    fn validate_after_compilation(&self) -> bool;
}

pub trait InternalShader {
    fn gl_id(&self) -> GlId;
}

#[derive(Debug,Clone,Copy)]
pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

#[derive(Debug,Clone,Copy)]
pub struct ShaderSource<'a>(pub ShaderType, pub &'a str);

#[derive(Debug)]
pub struct Shader {
    gl_id: GlId,
}

impl Shader {
    pub fn new<C: ShaderCreationSupport>(support: &mut C,
                                         shader_source: &ShaderSource)
                                         -> ReglResult<Shader> {
        let gl_id = glcall!(CreateShader(gl_shader_type(shader_source.0)));

        try!(add_shader_source(gl_id, shader_source.1));
        glcall!(CompileShader(gl_id));
        if support.validate_after_compilation() && !compiled(gl_id) {
            return Err(ReglError::ShaderCompilationError(info_log(gl_id)));
        }

        Ok(Shader { gl_id: gl_id })
    }

    pub fn info_log(&self) -> String {
        info_log(self.gl_id)
    }
}

impl InternalShader for Shader {
    fn gl_id(&self) -> GlId {
        self.gl_id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        glcall!(DeleteShader(self.gl_id));
    }
}

fn add_shader_source(gl_id: GlId, source: &str) -> ReglResult<()> {
    let c_source = try!(CString::new(source));
    let len = c_source.to_bytes().len() as GLint;
    glcall!(ShaderSource(gl_id, 1, &c_source.as_ptr(), &len));
    Ok(())
}

fn compiled(gl_id: GlId) -> bool {
    let mut value = 0;
    glcall!(GetShaderiv(gl_id, COMPILE_STATUS, &mut value));
    value != 0
}

fn info_log_length(gl_id: GlId) -> GLint {
    let mut value = 0;
    glcall!(GetShaderiv(gl_id, INFO_LOG_LENGTH, &mut value));
    value
}

fn info_log(gl_id: GlId) -> String {
    let len = info_log_length(gl_id);
    let mut actual_len = 0;
    let mut log = vec![0u8; len as usize];
    // len - 1 because info_log_length returns the value *including* null terminator
    glcall!(GetShaderInfoLog(gl_id, len - 1, &mut actual_len, log.as_mut_ptr() as *mut i8));
    log.truncate(actual_len as usize);
    String::from_utf8_lossy(&log[..]).into_owned()
}

fn gl_shader_type(shader_type: ShaderType) -> GLenum {
    match shader_type {
        ShaderType::VertexShader => ::gl::VERTEX_SHADER,
        ShaderType::FragmentShader => ::gl::FRAGMENT_SHADER,
    }
}
