use crate::{ast::TypingType, ty_env::TyIEnv, Page, SpellBook};
use wiz_arena::{Arena, DeclarationId};
use wiz_result::Result;
use wiz_session::Session;

pub(crate) fn collect_items(
    spell_book: &SpellBook,
    arena: &mut Arena,
    session: &mut Session,
) -> Result<()> {
    let ty_env = TyIEnv::root();
    ty_env.new_scope(|ty_env| {
        collect_item_internal(
            ty_env,
            &spell_book.page,
            &spell_book.name,
            &DeclarationId::ROOT,
            arena,
            session,
        )
    })
}

fn collect_item_internal(
    mut ty_env: TyIEnv,
    page: &Page,
    name: &str,
    parent: &DeclarationId,
    arena: &mut Arena,
    session: &mut Session,
) -> Result<()> {
    let parent_item = arena.get_mut_by_id(parent).unwrap();
    let maybe_current_item_id = parent_item.get_child(name).unwrap();
    let maybe_current_item_id = maybe_current_item_id.iter().copied().collect::<Vec<_>>();
    let current_item_id = maybe_current_item_id.first().unwrap();

    for u in page.uses.iter() {
        let ty = if let Some(name) = u.namespace.first() {
            let mut namespace_id = None;
            let ty = ty_env.lookup(name);
            if let Some(TypingType::Namespace(id)) = ty {
                let item_id = arena.resolve_declaration_id(id, &u.namespace[1..]);
                namespace_id = item_id;
            };
            if let Some(i) = namespace_id {
                arena
                    .resolve_declaration_id(i, &[&u.name])
                    .map(|i| TypingType::Namespace(i))
            } else {
                None
            }
        } else {
            ty_env.lookup(&u.name)
        };
        if let Some(ty) = ty {
            if let Some(name) = &u.alias {
                ty_env.extend(name, ty);
            } else {
                ty_env.extend(&u.name, ty);
            }
        }
    }

    for var_def in page.var_defs.iter() {}

    for function_def in page.function_defs.iter() {
        // arena.register_function()
    }

    for struct_def in page.struct_defs.iter() {}

    for extension_def in page.extension_defs.iter() {}

    for (child_name, child_page) in page.pages.iter() {
        ty_env.new_scope(|ty_env| {
            collect_item_internal(
                ty_env,
                child_page,
                child_name,
                current_item_id,
                arena,
                session,
            )
        })?;
    }
    Ok(())
}
