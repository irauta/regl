
use std::rc::Rc;
use std::fmt::Debug;
use ::id::{Id,GenerateId};
use ::ReglResult;
use ::GlId;
use ::tracker::BindIf;
use ::resource::ResourceCreationSupport;

pub trait FramebufferSupport : BindIf<DrawFramebufferTag> + Debug {

}

pub trait FramebufferInternal {
    fn bind(&self);
}

#[allow(dead_code)]
pub struct DrawFramebufferTag;

#[derive(Debug)]
pub struct Framebuffer {
    shared_context: Rc<FramebufferSupport>,
    uid: Id,
    gl_id: GlId,
}

impl Framebuffer {
    pub fn new<C: ResourceCreationSupport>(support: &mut C) -> ReglResult<Framebuffer> {
        let mut gl_id = 0;
        glcall!(GenFramebuffers(1, &mut gl_id));
        let framebuffer = Framebuffer {
            shared_context: support.get_shared_context(),
            uid: support.generate_id(),
            gl_id: gl_id,
        };
        framebuffer.bind();
        // TODO: Attach stuff to framebuffer
        // TODO: Check for completeness
        Ok(framebuffer)
    }

    pub fn clear(&self) {
        self.bind();
        glcall!(Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT | STENCIL_BUFFER_BIT));
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.bind();
        glcall!(ClearColor(r, g, b, a));
    }

    fn gl_bind(&self) {
        glcall!(BindFramebuffer(DRAW_FRAMEBUFFER, self.gl_id));
    }
}

impl FramebufferInternal for Framebuffer {
    fn bind(&self) {
        self.shared_context.bind_if(&self.uid, &|| self.gl_bind());
    }
}

pub fn create_default_framebuffer<C: ResourceCreationSupport>(support: &mut C) -> Framebuffer {
    let uid = support.generate_id();
    Framebuffer {
        shared_context: support.get_shared_context(),
        uid: uid,
        gl_id: 0,
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        if self.gl_id != 0 {
            glcall!(DeleteFramebuffers(1, &self.gl_id));
        }
    }
}
