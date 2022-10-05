mod scanner;

use scanner::AstScanner;
use wiz_arena::Arena;
use wiz_syntax::syntax::file::SourceSet;

pub fn detect_type_and_namespace(arena: &mut Arena, source_set: &SourceSet) {
    let scanner = AstScanner::new(arena);
    scanner.start(source_set);
}
