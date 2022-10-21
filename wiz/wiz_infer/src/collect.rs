use crate::{Page, SpellBook};
use wiz_arena::{Arena, DeclarationId};
use wiz_result::Result;
use wiz_session::Session;

pub(crate) fn collect_items(
    spell_book: &SpellBook,
    arena: &mut Arena,
    session: &mut Session,
) -> Result<()> {
    collect_item_internal(
        &spell_book.page,
        &spell_book.name,
        &DeclarationId::ROOT,
        arena,
        session,
    )
}

fn collect_item_internal(
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
    for (child_name, child_page) in page.pages.iter() {
        collect_item_internal(child_page, child_name, current_item_id, arena, session)?;
    }
    Ok(())
}
