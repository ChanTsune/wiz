use wiz_arena::Arena;
use wiz_session::Session;
use wiz_syntax::syntax::file::SourceSet;

pub struct AstScanner<'a> {
    session: &'a Session,
    arena: &'a mut Arena,
}

impl<'a> AstScanner<'a> {
    pub(crate) fn new(session: &'a Session, arena: &'a mut Arena) -> Self {
        Self { session, arena }
    }
}

impl<'a> AstScanner<'a> {
    pub(crate) fn start(&self, source_set: &SourceSet) {}
}
