
pub mod shared;

use std::rc::Rc;
use self::shared::{SharedContext,new_shared_context};
use ::id::{Id,IdGenerator,GenerateId};
use ::resource::ResourceCreationSupport;
use ::framebuffer::{self,Framebuffer};

pub struct Context {
    id_gen: IdGenerator,
    shared_context: Rc<SharedContext>,
    default_framebuffer: Framebuffer,
}

impl Context {
    pub fn new() -> Context {
        let mut booter = ContextBooter {
            id_gen: IdGenerator::new(),
            shared_context: Rc::new(new_shared_context()),
        };
        let default_framebuffer = framebuffer::create_default_framebuffer(&mut booter);
        Context {
            id_gen: booter.id_gen,
            shared_context: booter.shared_context,
            default_framebuffer: default_framebuffer,
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
