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
    pub const fn new(module: ModuleId, id: usize) -> Self {
        Self { module, id }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct NodeIdGenerator {
    module_id: ModuleId,
    id: usize,
}

impl NodeIdGenerator {
    pub const fn new(module_id: ModuleId) -> Self {
        Self { module_id, id: 0 }
    }

    pub fn generate(&mut self) -> NodeId {
        let node_id = NodeId {
            module: self.module_id,
            id: self.id,
        };
        self.id += 1;
        node_id
    }
}
