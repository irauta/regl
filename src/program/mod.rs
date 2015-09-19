
use std::rc::Rc;
use std::fmt::Debug;
use ::id::{Id,GenerateId};
use ::ReglResult;
use ::GlId;
use ::tracker::BindIf;
use ::resource::ResourceCreationSupport;

pub mod shader;

pub trait ProgramSupport : BindIf<Program> + Debug {}

#[derive(Debug)]
pub struct Program {
    shared_context: Rc<ProgramSupport>,
    uid: Id,
    gl_id: GlId,
}

impl Program {
    pub fn new(support: &mut ResourceCreationSupport) -> ReglResult<Program> {
        let gl_id = glcall!(CreateProgram());
        let program = Program {
            shared_context: support.get_shared_context(),
            uid: support.generate_id(),
            gl_id: gl_id,
        };
        Ok(program)
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        glcall!(DeleteProgram(self.gl_id));
    }
}
