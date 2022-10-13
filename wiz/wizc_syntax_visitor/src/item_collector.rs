use wiz_arena::Arena;
use wiz_hir::typed_file::TypedSpellBook;
use wiz_session::Session;
use wiz_syntax::syntax::file::SourceSet;

pub(crate) struct AstItemCollector<'a> {
    session: &'a Session,
    arena: &'a Arena,
}

impl<'a> AstItemCollector<'a> {
    pub(crate) fn new(session: &'a Session, arena: &'a Arena) -> Self {
        Self { session, arena }
    }
}

impl<'a> AstItemCollector<'a> {
    pub(crate) fn start(&self, module: &mut TypedSpellBook, source_set: &SourceSet) {}
}
