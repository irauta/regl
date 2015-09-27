
use std::cell::Cell;
use ::id::{Id,WeakId};

#[derive(Debug)]
pub struct SimpleTracker {
    current: Cell<WeakId>,
}

impl SimpleTracker {
    pub fn new() -> SimpleTracker {
        SimpleTracker {
            current: Cell::new(WeakId::empty()),
        }
    }

    pub fn bind_if(&self, uid: &Id, bind: &Fn()) {
        if self.current.get() != *uid {
            bind();
            self.current.set(uid.weak());
        }
    }

    pub fn bind_none(&self) {
        self.current.set(WeakId::empty());
    }
}

/// The type parameter on the trait is not actively used, but works as a discriminator,
/// so that a single struct can implement this trait for several types.
pub trait BindIf<T> {
    fn bind_if(&self, uid: &Id, bind: &Fn());
}

pub trait BindNone<T> {
    fn bind_none(&self);
}
