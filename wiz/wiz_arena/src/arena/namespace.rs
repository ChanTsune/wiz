use wiz_hir::typed_use::TypedUse;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArenaNamespace {
    pub uses: Vec<TypedUse>,
}

impl ArenaNamespace {
    pub(crate) fn new() -> Self {
        Self { uses: Vec::new() }
    }
}
