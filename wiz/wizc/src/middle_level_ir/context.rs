use std::collections::HashMap;
use wiz_mir::ml_decl::MLStruct;
use wiz_mir::ml_type::MLValueType;
use crate::high_level_ir::typed_annotation::TypedAnnotations;

pub(crate) struct HLIR2MLIRContext {
    pub(crate) declaration_annotations: HashMap<String, TypedAnnotations>,
    pub(crate) structs: HashMap<MLValueType, MLStruct>,
    pub(crate) current_name_space: Vec<String>,
}

impl HLIR2MLIRContext {
    pub(crate) fn new() -> Self {
        Self {
            declaration_annotations: Default::default(),
            structs: Default::default(),
            current_name_space: vec![],
        }
    }

    pub(crate) fn set_declaration_annotations(&mut self, name: String, a: TypedAnnotations) {
        self.declaration_annotations.insert(name, a);
    }

    pub(crate) fn declaration_has_annotation(
        &self,
        declaration_name: &str,
        annotation: &str,
    ) -> bool {
        let an = self.declaration_annotations.get(declaration_name);
        an.map(|a| a.has_annotate(annotation))
            .unwrap_or_else(|| false)
    }

    pub(crate) fn get_struct(&self, typ: &MLValueType) -> &MLStruct {
        self.structs.get(typ).unwrap_or_else(|| panic!("{:?}", typ))
    }

    pub(crate) fn struct_has_field(&self, typ: &MLValueType, field_name: &str) -> bool {
        self.get_struct(typ)
            .fields
            .iter()
            .any(|f| f.name == *field_name)
    }

    pub(crate) fn add_struct(&mut self, typ: MLValueType, struct_: MLStruct) {
        self.structs.insert(typ, struct_);
    }

    pub(crate) fn push_name_space(&mut self, name: String) {
        self.current_name_space.push(name)
    }

    pub(crate) fn pop_name_space(&mut self) {
        self.current_name_space.pop();
    }
}

