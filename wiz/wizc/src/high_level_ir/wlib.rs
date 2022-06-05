use crate::high_level_ir::declaration_id::DeclarationId;
use crate::ResolverArena;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::Path;
use wiz_hir::typed_decl::TypedDeclKind;
use wiz_hir::typed_file::TypedSourceSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WLib {
    pub typed_ir: TypedSourceSet,
}

impl WLib {
    pub fn new(typed_ir: TypedSourceSet) -> WLib {
        WLib { typed_ir }
    }

    pub fn read_from(path: &Path) -> WLib {
        let file = std::fs::read_to_string(path).unwrap();
        let lib: WLib = serde_json::from_str(&file).unwrap();
        lib
    }

    pub fn write_to(&self, path: &Path) {
        let file = serde_json::to_string(self).unwrap();
        std::fs::write(path, file).unwrap();
    }

    pub fn apply_to(&self, arena: &mut ResolverArena) -> Result<(), String> {
        let namespace_id = DeclarationId::ROOT;
        self._apply_to(&namespace_id, &self.typed_ir, arena)
    }

    fn _apply_to(
        &self,
        parent: &DeclarationId,
        source_set: &TypedSourceSet,
        arena: &mut ResolverArena,
    ) -> Result<(), String> {
        match source_set {
            TypedSourceSet::File(f) => {
                let id = arena
                    .register_namespace(parent, &f.name, Default::default())
                    .unwrap();
                for decl in &f.body {
                    match &decl.kind {
                        TypedDeclKind::Var(v) => {
                            arena.register_value(&id, &v.name, v.type_.clone().unwrap(), decl.annotations.clone());
                        }
                        TypedDeclKind::Fun(f) => {
                            arena.register_value(&id, &f.name, f.type_().unwrap(),decl.annotations.clone());
                        }
                        TypedDeclKind::Struct(s) => {
                            arena.register_struct(&id, &s.name, decl.annotations.clone());
                        }
                        TypedDeclKind::Class => {}
                        TypedDeclKind::Enum => {}
                        TypedDeclKind::Protocol(p) => {
                            arena.register_struct(&id, &p.name, decl.annotations.clone());
                        }
                        TypedDeclKind::Extension(_) => {}
                    };
                }
            }
            TypedSourceSet::Dir { name, items } => {
                let id = arena
                    .register_namespace(parent, name, Default::default())
                    .unwrap();

                items
                    .iter()
                    .try_for_each(|f| self._apply_to(&id, f, arena))?;
            }
        }
        Ok(())
    }
}
