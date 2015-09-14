
use std::rc::Rc;
use ::id::{Id,GenerateId};
use ::ReglResult;
use ::GlId;
use ::tracker::BindIf;
use ::resource::ResourceCreationSupport;

pub trait VertexArraySupport : BindIf<VertexArray> {

}

pub struct VertexArray {
    shared_context: Rc<VertexArraySupport>,
    uid: Id,
    gl_id: GlId,
}

impl VertexArray {
    pub fn new(support: &mut ResourceCreationSupport) -> ReglResult<VertexArray> {
        let mut gl_id = 0;
        glcall!(GenVertexArrays(1, &mut gl_id));
        let vertex_array = VertexArray {
            shared_context: support.get_shared_context(),
            uid: support.generate_id(),
            gl_id: gl_id
        };
        vertex_array.bind();
        // TODO: Attach stuff to vertex array
        Ok(vertex_array)
    }

    fn bind(&self) {
        self.shared_context.bind_if(&self.uid, &|| self.gl_bind());
    }

    fn gl_bind(&self) {
        glcall!(BindVertexArray(self.gl_id));
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        glcall!(DeleteVertexArrays(1, &self.gl_id));
    }
}
