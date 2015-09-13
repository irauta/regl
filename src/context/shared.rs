
use ::id::Id;
use ::tracker::{SimpleTracker,BindIf};
use ::framebuffer::{FramebufferSupport,DrawFramebufferTag};
use ::buffer::BufferSupport;

pub struct SharedContext {
    draw_framebuffer: SimpleTracker,
}

pub fn new_shared_context() -> SharedContext {
    SharedContext {
        draw_framebuffer: SimpleTracker::new(),
    }
}


impl BindIf<DrawFramebufferTag> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn()) {
        self.draw_framebuffer.bind_if(uid, bind)
    }
}

/*impl BindIf<Framebuffer> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn() -> Result<(), ReglError>) -> Result<(), ReglError>
    {
        self.current_framebuffer.bind_if(uid, bind)
    }
}*/

impl FramebufferSupport for SharedContext {}

impl BufferSupport for SharedContext {}
