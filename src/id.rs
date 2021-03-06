
use std::cmp::{PartialEq, Eq};

pub type GlId = ::gl::types::GLuint;

/// Helper alias, does not represent a GL id
type IdValue = u32;

/// Id helps identifying objects.
///
/// OpenGL may reuse old ids, so problems might arise when tracking which object is bound.
/// Every Id generated by the same IdGenerator is unique (until the IdGenerator's internal counter
/// wraps around). Therefore, you really shouldn't have more than one generator.
///
/// Ids are not copyable to avoid certain kinds of bugs. For the purposes of tracking what's bound
/// a WeakId should be used. (Of course other uses than just tracking currently bound are allowed.)
#[derive(Debug)]
pub struct Id {
    unique_id: IdValue,
}

impl Id {
    pub fn weak(&self) -> WeakId {
        WeakId { id: self.unique_id }
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Id) -> bool {
        self.unique_id == other.unique_id
    }
}

impl PartialEq<WeakId> for Id {
    fn eq(&self, other: &WeakId) -> bool {
        self.unique_id == other.id
    }
}

impl Eq for Id {}

/// Weak counterpart of the Id. May be copied, so there's less guarantees of uniqueness.
#[derive(Debug,Clone,Copy)]
pub struct WeakId {
    id: IdValue,
}

impl WeakId {
    /// Returns a WeakId that is not equal with any Id.
    pub fn empty() -> WeakId {
        WeakId { id: 0 }
    }
}

impl PartialEq for WeakId {
    fn eq(&self, other: &WeakId) -> bool {
        self.id == other.id
    }
}

impl PartialEq<Id> for WeakId {
    fn eq(&self, other: &Id) -> bool {
        other == self
    }
}

impl Eq for WeakId {}

/// IdGenerator is the current way of obtaining new Ids. Please don't make more than one, to
/// keep things sane!
#[derive(Debug)]
pub struct IdGenerator {
    counter: IdValue,
}

impl IdGenerator {
    /// Create a new IdGenerator.
    pub fn new() -> IdGenerator {
        IdGenerator { counter: 0 }
    }

    /// Create a new Id.
    pub fn generate_id(&mut self) -> Id {
        self.counter += 1;
        Id { unique_id: self.counter }
    }
}

/// A type that owns an IdGenerator may implement this trait.
pub trait GenerateId {
    fn generate_id(&mut self) -> Id;
}

impl GenerateId for IdGenerator {
    fn generate_id(&mut self) -> Id {
        IdGenerator::generate_id(self)
    }
}
