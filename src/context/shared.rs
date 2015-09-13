
use ::id::Id;
use ::ReglError;
use ::tracker::{SimpleTracker,BindIf};
use ::framebuffer::{Framebuffer,FramebufferSupport};
use ::buffer::BufferSupport;

pub struct SharedContext {
    current_framebuffer: SimpleTracker,
}

pub fn new_shared_context() -> SharedContext {
    SharedContext {
        current_framebuffer: SimpleTracker::new(),
    }
}


impl BindIf<Framebuffer> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn())
    {
        self.current_framebuffer.bind_if(uid, bind)
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
