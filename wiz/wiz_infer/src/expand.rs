use crate::{Page, SpellBook, Use};
use wiz_arena::{Arena, DeclarationId};
use wiz_data_structure::annotation::Annotations;
use wiz_result::Result;
use wiz_session::Session;
use wiz_syntax::syntax::declaration::DeclKind;
use wiz_syntax::syntax::{FileSyntax, WizFile};
use wiz_syntax_parser::parser::wiz::parse_from_file_path;

/// expand ast and collect `namespace`, `type` and `use`
pub(crate) fn expand_ast(
    f: WizFile,
    arena: &mut Arena,
    session: &mut Session,
) -> Result<SpellBook> {
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
        .register_namespace(parent, name, Annotations::default())
        .unwrap();
    for item in f.body.into_iter() {
        match item.kind {
            DeclKind::Var(var) => {
                page.var_defs.push(var);
            }
            DeclKind::Fun(function) => {
                page.function_defs.push(function);
            }
            DeclKind::Struct(s) => {
                let name = s.name.token();
                match s.struct_keyword.token().as_str() {
                    "struct" => arena.register_struct(&parent, &name, Annotations::default()),
                    "protocol" => arena.register_protocol(&parent, &name, Annotations::default()),
                    token => unreachable!("{}", token),
                };
                page.struct_defs.push(s);
            }
            DeclKind::ExternC(_) => todo!(),
            DeclKind::Enum { .. } => todo!(),
            DeclKind::Module((name, body)) => {
                let mut child_page = Page::empty();

                let body = match body {
                    None => {
                        let mut s = session.local_spell_book_root().to_owned();
                        let fqn = arena.resolve_fully_qualified_name(&parent);
                        for n in &fqn[1..] {
                            s = s.join(n);
                        }
                        s.set_extension("wiz");
                        println!("Module: {}", s.display());
                        let file = parse_from_file_path(&session.parse_session, s, Some(&name))?;
                        file.syntax
                    }
                    Some(body) => body,
                };
                expand_ast_internal(&mut child_page, &name, body, &parent, arena, session)?;

                page.pages.insert(name.clone(), child_page);
            }
            DeclKind::Extension(extension) => {
                page.extension_defs.push(extension);
            }
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
