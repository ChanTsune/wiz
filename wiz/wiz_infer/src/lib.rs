mod ast;
mod collect;
pub mod error;
mod expand;
mod infer;
pub mod result;
mod ty_env;

use crate::ast::{Page, SpellBook, Use};
use crate::collect::collect_items;
use crate::expand::expand_ast;
use infer::infer_source_set;
pub use ty_env::TypeEnvironment;
use wiz_arena::Arena;
use wiz_hir::typed_file::TypedSpellBook;
use wiz_result::Result;
use wiz_session::Session;
use wiz_syntax::syntax::WizFile;

pub fn run(
    source_set: WizFile,
    arena: &mut Arena,
    session: &mut Session,
) -> Result<TypedSpellBook> {
    let sb = expand_ast(source_set, arena, session)?;
    collect_items(&sb, arena, session)?;
    Ok(infer_source_set(sb, arena, &TypeEnvironment::root())?)
}
