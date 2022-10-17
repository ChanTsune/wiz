use wiz_arena::Arena;
use wiz_session::Session;
use wiz_syntax::syntax::file::SourceSet;

pub(crate) struct AstItemCollector {}

impl AstItemCollector {
    pub(crate) fn new(session: &Session, arena: &Arena) -> Self {
        Self {}
    }
}

impl AstItemCollector {
    pub(crate) fn start(&self, source_set: &SourceSet) {}
}
