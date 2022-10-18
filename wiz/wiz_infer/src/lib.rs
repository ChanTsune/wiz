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
    let mut sb = SpellBook::empty(source_set.name.clone());
    collect_namespace_and_type(
        &mut sb.page,
        &source_set.name,
        &source_set.syntax,
        &DeclarationId::ROOT,
        arena,
        session,
    )?;
    infer_source_set(source_set, arena, &TypeEnvironment::root())
}

/// collect `namespace`, `type` and `use`
fn collect_namespace_and_type(
    parent_page: &mut Page,
    name: &str,
    f: &FileSyntax,
    parent: &DeclarationId,
    arena: &mut Arena,
    session: &mut Session,
) -> Result<()> {
    let parent = arena
        .register_namespace(parent, name, TypedAnnotations::default())
        .unwrap();
    let mut page = Page::empty();
    for item in f.body.iter() {
        if let DeclKind::Use(u) = &item.kind {
            page.uses.push(Use::new(
                u.package_name
                    .as_ref()
                    .map(|i| i.names.iter().map(|it| it.name.token()).collect::<Vec<_>>())
                    .unwrap_or_default(),
                u.used_name.token(),
                u.alias.as_ref().map(|it| it.name.token()),
            ));
        } else if let DeclKind::Struct(s) = &item.kind {
            let name = s.name.token();
            match s.struct_keyword.token().as_str() {
                "struct" => arena.register_struct(&parent, &name, TypedAnnotations::default()),
                "protocol" => arena.register_protocol(&parent, &name, TypedAnnotations::default()),
                token => unreachable!("{}", token),
            };
        } else if let DeclKind::Module((name, body)) = &item.kind {
            let mut child_page = Page::empty();
            collect_namespace_and_type(
                &mut child_page,
                name,
                body.as_ref().unwrap(),
                &parent,
                arena,
                session,
            )?;
            page.pages.insert(name.clone(), child_page);
        }
    }
    parent_page.pages.insert(name.to_string(), page);
    Ok(())
}
