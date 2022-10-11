mod scanner;

use scanner::AstScanner;
use wiz_arena::Arena;
use wiz_session::Session;
use wiz_syntax::syntax::file::SourceSet;

pub fn collect_type_and_namespace(session: &Session, arena: &mut Arena, source_set: &SourceSet) {
    let mut scanner = AstScanner::new(session, arena);
    scanner.start(source_set);
}
