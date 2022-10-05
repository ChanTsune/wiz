use wiz_arena::Arena;
use wiz_syntax::syntax::file::SourceSet;

pub struct AstScanner<'a> {
    arena: &'a mut Arena,
}

impl<'a> AstScanner<'a> {
    pub(crate) fn new(arena: &'a mut Arena) -> Self {
        Self { arena }
    }
}

impl<'a> AstScanner<'a> {
    pub(crate) fn start(source_set: &SourceSet) {}
}
