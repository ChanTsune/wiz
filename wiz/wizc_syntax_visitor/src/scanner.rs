use wiz_arena::{Arena, DeclarationId};
use wiz_session::Session;
use wiz_syntax::syntax::file::SourceSet;

pub struct AstScanner<'a> {
    session: &'a Session,
    arena: &'a mut Arena,
    namespace_id: DeclarationId,
}

impl<'a> AstScanner<'a> {
    pub(crate) fn new(session: &'a Session, arena: &'a mut Arena) -> Self {
        Self {
            session,
            arena,
            namespace_id: DeclarationId::ROOT,
        }
    }
}

impl<'a> AstScanner<'a> {
    fn push_namespace<T, F: FnOnce(&mut Self) -> T>(&mut self, name: &str, f: F) -> T {
        let parent = self.namespace_id;

        self.namespace_id = self
            .arena
            .resolve_declaration_id(parent, &[name])
            .unwrap_or_else(|| {
                self.arena
                    .register_namespace(&parent, name, Default::default())
                    .unwrap_or_else(|| panic!("Can not create {}", name))
            });

        let result = f(self);

        self.namespace_id = parent;
        result
    }
}

impl<'a> AstScanner<'a> {
    pub(crate) fn start(&self, source_set: &SourceSet) {}
}
