#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct ModuleId(usize);

impl ModuleId {
    pub(crate) const DUMMY: Self = Self::new(usize::MAX);
    pub const fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct NodeId {
    module: ModuleId,
    id: usize,
}

impl NodeId {
    pub fn new(module: ModuleId, id: usize) -> Self {
        Self { module, id }
    }
}
