mod item_collector;
mod type_collector;

use crate::item_collector::AstItemCollector;
use type_collector::AstTypeCollector;
use wiz_arena::Arena;
use wiz_hir::typed_file::TypedSpellBook;
use wiz_session::Session;
use wiz_syntax::syntax::file::SourceSet;

pub fn collect_type_and_namespace(session: &Session, arena: &mut Arena, source_set: &SourceSet) {
    let mut scanner = AstTypeCollector::new(session, arena);
    scanner.start(source_set);
}

pub fn collect_items(
    session: &Session,
    arena: &mut Arena,
    module: &mut TypedSpellBook,
    source_set: &SourceSet,
) {
    let mut preloader = AstItemCollector::new(session, arena);
    preloader.start(module, source_set)
}
