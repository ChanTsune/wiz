use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::Path;
use wiz_arena::{Arena, DeclarationId, DeclarationItemKind};
use wiz_hir::typed_decl::TypedDeclKind;
use wiz_hir::typed_file::TypedFile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WLib {
    pub typed_ir: TypedFile,
}

impl WLib {
    pub fn new(typed_ir: TypedFile) -> WLib {
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

    pub fn apply_to(&self, arena: &mut Arena) -> Result<(), String> {
        let namespace_id = DeclarationId::ROOT;
        self._apply_to(&namespace_id, &self.typed_ir, arena)
    }

    fn _apply_to(
        &self,
        parent: &DeclarationId,
        f: &TypedFile,
        arena: &mut Arena,
    ) -> Result<(), String> {
        let id = arena
            .register_namespace(parent, &f.name, Default::default())
            .unwrap();
        for decl in &f.body {
            match &decl.kind {
                TypedDeclKind::Var(v) => {
                    arena.register_value(
                        &id,
                        &v.name,
                        v.type_.clone().unwrap(),
                        decl.annotations.clone(),
                    );
                }
                TypedDeclKind::Fun(f) => {
                    arena.register_function(
                        &id,
                        &f.name,
                        f.type_(),
                        f.type_params.clone(),
                        f.body.clone(),
                        decl.annotations.clone(),
                    );
                }
                TypedDeclKind::Struct(s) => {
                    let id = arena
                        .register_struct(&id, &s.name, decl.annotations.clone())
                        .unwrap();
                    let item = arena.get_mut_by_id(&id).unwrap();
                    if let DeclarationItemKind::Type(rs) = &mut item.kind {
                        rs.stored_properties.extend(
                            s.stored_properties
                                .iter()
                                .cloned()
                                .map(|t| (t.name, t.type_)),
                        )
                    }
                    for member_function in s.member_functions.iter() {
                        arena.register_function(
                            &id,
                            &member_function.name,
                            member_function.type_(),
                            member_function.type_params.clone(),
                            member_function.body.clone(),
                            Default::default(),
                        );
                    }
                }
                TypedDeclKind::Module(m) => {
                    self._apply_to(&id, m, arena)?;
                }
                TypedDeclKind::Enum => {}
                TypedDeclKind::Protocol(p) => {
                    arena.register_struct(&id, &p.name, decl.annotations.clone());
                }
                TypedDeclKind::Extension(_) => {}
            };
        }
        Ok(())
    }
}
