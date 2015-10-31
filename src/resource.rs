
use std::rc::Rc;
use id::GenerateId;
use context::shared::SharedContext;

pub trait ResourceCreationSupport : GenerateId {
    fn get_shared_context(&mut self) -> Rc<SharedContext>;
}
