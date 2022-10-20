mod ast;
pub mod error;
mod infer;
pub mod result;
mod ty_env;

use crate::ast::{Page, SpellBook, Use};
use crate::ty_env::TyEnv;
use infer::infer_source_set;
use result::Result;
pub use ty_env::TypeEnvironment;
use wiz_arena::{Arena, DeclarationId};
use wiz_hir::typed_annotation::TypedAnnotations;
use wiz_hir::typed_file::TypedSpellBook;
use wiz_session::Session;
use wiz_syntax::syntax::declaration::DeclKind;
use wiz_syntax::syntax::{FileSyntax, WizFile};

pub fn run(
    source_set: WizFile,
    arena: &mut Arena,
    session: &mut Session,
) -> Result<TypedSpellBook> {
    let sb = expand_ast(source_set.clone(), arena, session)?;
    infer_source_set(source_set, arena, &TypeEnvironment::root())
}

/// expand ast and collect `namespace`, `type` and `use`
fn expand_ast(f: WizFile, arena: &mut Arena, session: &mut Session) -> Result<SpellBook> {
    let WizFile { name, syntax } = f;
    let mut sb = SpellBook::empty(name);
    expand_ast_internal(
        &mut sb.page,
        &sb.name,
        syntax,
        &DeclarationId::ROOT,
        arena,
        session,
    )?;
    Ok(sb)
}

fn expand_ast_internal(
    page: &mut Page,
    name: &str,
    f: FileSyntax,
    parent: &DeclarationId,
    arena: &mut Arena,
    session: &mut Session,
) -> Result<()> {
    let parent = arena
        .register_namespace(parent, name, TypedAnnotations::default())
        .unwrap();
    for item in f.body.into_iter() {
        match item.kind {
            DeclKind::Var(_) => {}
            DeclKind::Fun(_) => {}
            DeclKind::Struct(s) => {
                let name = s.name.token();
                match s.struct_keyword.token().as_str() {
                    "struct" => arena.register_struct(&parent, &name, TypedAnnotations::default()),
                    "protocol" => {
                        arena.register_protocol(&parent, &name, TypedAnnotations::default())
                    }
                    token => unreachable!("{}", token),
                };
            }
            DeclKind::ExternC(_) => {}
            DeclKind::Enum { .. } => {}
            DeclKind::Module((name, body)) => {
                let mut child_page = Page::empty();
                match body {
                    None => todo!("expand namespace ast {}", name),
                    Some(body) => {
                        expand_ast_internal(&mut child_page, &name, body, &parent, arena, session)?;
                    }
                }
                page.pages.insert(name.clone(), child_page);
            }
            DeclKind::Extension(_) => {}
            DeclKind::Use(u) => {
                page.uses.push(Use::new(
                    u.package_name
                        .as_ref()
                        .map(|i| i.names.iter().map(|it| it.name.token()).collect::<Vec<_>>())
                        .unwrap_or_default(),
                    u.used_name.token(),
                    u.alias.as_ref().map(|it| it.name.token()),
                ));
            }
        }
    }
    Ok(())
}
