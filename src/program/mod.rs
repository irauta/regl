
use std::rc::Rc;
use std::fmt::Debug;
use ::id::{Id,GenerateId};
use ::ReglResult;
use ::GlId;
use ::tracker::BindIf;
use ::resource::ResourceCreationSupport;

use self::shader::Shader;
pub use self::shader::{ShaderType,ShaderSource};

mod shader;

pub trait ProgramSupport : BindIf<Program> + Debug {}

#[derive(Debug)]
pub struct Program {
    shared_context: Rc<ProgramSupport>,
    uid: Id,
    gl_id: GlId,
}

impl Program {
    pub fn new(support: &mut ResourceCreationSupport, shader_sources: &[ShaderSource]) -> ReglResult<Program> {
        let shaders: Vec<Shader> = try!(shader_sources.iter().map(Shader::new).collect());

        let gl_id = glcall!(CreateProgram());

        for shader in shaders {
            glcall!(AttachShader(gl_id, shader.gl_id()));
        }

        glcall!(LinkProgram(gl_id));

        Ok(Program {
            shared_context: support.get_shared_context(),
            uid: support.generate_id(),
            gl_id: gl_id,
        })
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        glcall!(DeleteProgram(self.gl_id));
    }
}
