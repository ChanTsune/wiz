use crate::high_level_ir::typed_annotation::TypedAnnotations;
use std::collections::HashMap;
use wiz_mir::ml_decl::MLStruct;
use wiz_mir::ml_type::MLValueType;

#[derive(Default, Debug)]
pub(crate) struct HLIR2MLIRContext {
    pub(crate) structs: HashMap<MLValueType, MLStruct>,
}

impl HLIR2MLIRContext {
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
}
