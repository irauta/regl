
pub mod shared;

use std::rc::Rc;
use self::shared::{SharedContext,new_shared_context};
use ::id::{Id,IdGenerator,GenerateId};
use ::resource::ResourceCreationSupport;
use ::buffer::BufferCreationSupport;
use ::framebuffer::{self,Framebuffer};
use ::vertex_array::{self,VertexArray};

pub struct Context {
    id_gen: IdGenerator,
    shared_context: Rc<SharedContext>,
    default_framebuffer: Framebuffer,
    default_vertex_array: Rc<VertexArray>,
}

impl Context {
    pub fn new() -> Context {
        let mut booter = ContextBooter {
            id_gen: IdGenerator::new(),
            shared_context: Rc::new(new_shared_context()),
        };
        let default_framebuffer = framebuffer::create_default_framebuffer(&mut booter);
        let default_vertex_array = Rc::new(vertex_array::create_default_vertex_array(&mut booter).unwrap());
        Context {
            id_gen: booter.id_gen,
            shared_context: booter.shared_context,
            default_framebuffer: default_framebuffer,
            default_vertex_array: default_vertex_array,
        }
    }

    pub fn default_framebuffer(&self) -> &Framebuffer {
        &self.default_framebuffer
    }
}

impl GenerateId for Context {
    fn generate_id(&mut self) -> Id {
        self.id_gen.generate_id()
    }
}

impl ResourceCreationSupport for Context {
    fn get_shared_context(&mut self) -> Rc<SharedContext> {
        self.shared_context.clone()
    }
}

impl BufferCreationSupport for Context {
    fn get_default_vertex_array(&mut self) -> Rc<VertexArray> {
        self.default_vertex_array.clone()
    }
}

struct ContextBooter {
    id_gen: IdGenerator,
    shared_context: Rc<SharedContext>,
}

impl GenerateId for ContextBooter {
    fn generate_id(&mut self) -> Id {
        self.id_gen.generate_id()
    }
}

impl ResourceCreationSupport for ContextBooter {
    fn get_shared_context(&mut self) -> Rc<SharedContext> {
        self.shared_context.clone()
    }
}
