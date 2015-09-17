
use ::id::Id;
use ::tracker::{SimpleTracker,BindIf};
use ::framebuffer::{FramebufferSupport,DrawFramebufferTag};
use ::buffer::{BufferSupport,VertexBufferTag,IndexBufferTag,UniformBufferTag};
use ::vertex_array::{VertexArray,VertexArraySupport};

#[derive(Debug)]
pub struct SharedContext {
    draw_framebuffer_tracker: SimpleTracker,
    vertex_array_tracker: SimpleTracker,
    vertex_buffer_tracker: SimpleTracker,
    index_buffer_tracker: SimpleTracker,
    uniform_buffer_tracker: SimpleTracker,
}

pub fn new_shared_context() -> SharedContext {
    SharedContext {
        draw_framebuffer_tracker: SimpleTracker::new(),
        vertex_array_tracker: SimpleTracker::new(),
        vertex_buffer_tracker: SimpleTracker::new(),
        index_buffer_tracker: SimpleTracker::new(),
        uniform_buffer_tracker: SimpleTracker::new(),
    }
}


impl BindIf<DrawFramebufferTag> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn()) {
        self.draw_framebuffer_tracker.bind_if(uid, bind)
    }
}

impl BindIf<VertexArray> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn()) {
        self.vertex_array_tracker.bind_if(uid, bind)
    }
}

impl BindIf<VertexBufferTag> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn()) {
        self.vertex_buffer_tracker.bind_if(uid, bind)
    }
}

impl BindIf<IndexBufferTag> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn()) {
        self.index_buffer_tracker.bind_if(uid, bind)
    }
}

impl BindIf<UniformBufferTag> for SharedContext {
    fn bind_if(&self, uid: &Id, bind: &Fn()) {
        self.uniform_buffer_tracker.bind_if(uid, bind)
    }
}

impl FramebufferSupport for SharedContext {}

impl VertexArraySupport for SharedContext {}

impl BufferSupport for SharedContext {}
