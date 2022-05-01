#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TypedModuleId(usize);

impl TypedModuleId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct TypedNodeId {
    module: TypedModuleId,
    id: usize,
}

impl TypedNodeId {
    pub fn new(module: TypedModuleId, id: usize) -> Self {
        Self { module, id }
    }
}
