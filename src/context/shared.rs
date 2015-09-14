
use ::id::Id;
use ::tracker::{SimpleTracker,BindIf};
use ::framebuffer::{FramebufferSupport,DrawFramebufferTag};
use ::buffer::{BufferSupport,VertexBufferTag,IndexBufferTag};
use ::vertex_array::{VertexArray,VertexArraySupport};

pub struct SharedContext {
    draw_framebuffer: SimpleTracker,
    vertex_array: SimpleTracker,
    vertex_buffer: SimpleTracker,
    index_buffer: SimpleTracker,
}

pub fn new_shared_context() -> SharedContext {
    SharedContext {
        draw_framebuffer: SimpleTracker::new(),
        vertex_array: SimpleTracker::new(),
        vertex_buffer: SimpleTracker::new(),
        index_buffer: SimpleTracker::new(),
    }
}


impl BindIf<DrawFramebufferTag> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn()) {
        self.draw_framebuffer.bind_if(uid, bind)
    }
}

impl BindIf<VertexArray> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn()) {
        self.vertex_array.bind_if(uid, bind)
    }
}

impl BindIf<VertexBufferTag> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn()) {
        self.vertex_buffer.bind_if(uid, bind)
    }
}

impl BindIf<IndexBufferTag> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn()) {
        // TODO: Do what's necessary to bind an index buffer!
        //self.vertex_array.bind_if(uid, bind)
    }
}

/*impl BindIf<Framebuffer> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn() -> Result<(), ReglError>) -> Result<(), ReglError>
    {
        self.current_framebuffer.bind_if(uid, bind)
    }
}*/

impl FramebufferSupport for SharedContext {}

impl VertexArraySupport for SharedContext {}

impl BufferSupport for SharedContext {}
