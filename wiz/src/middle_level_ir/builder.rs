use crate::middle_level_ir::ml_decl::{MLArgDef, MLDecl, MLField, MLFun, MLStruct, MLVar};
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::ml_type::MLValueType;
use std::collections::HashMap;

pub struct MLIRModuleBuilder {
    name: String,
    functions: HashMap<String, MLFun>,
    variables: HashMap<String, MLVar>,
    structs: HashMap<String, MLStruct>,
}

impl MLIRModuleBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: Default::default(),
            variables: Default::default(),
            structs: Default::default(),
        }
    }

    pub fn create_function(
        &mut self,
        name: String,
        args: Vec<MLArgDef>,
        return_type: MLValueType,
    ) -> Option<&mut MLFun> {
        self.add_function(MLFun {
            modifiers: vec![],
            name: name.clone(),
            arg_defs: args,
            return_type,
            body: None,
        })
    }

    pub fn add_function(&mut self, fun: MLFun) -> Option<&mut MLFun> {
        let name = fun.name.clone();
        self.functions.insert(name.clone(), fun)?;
        self.get_function(&name)
    }

    pub fn get_function(&mut self, name: &String) -> Option<&mut MLFun> {
        self.functions.get_mut(name)
    }

    pub fn create_struct(&mut self, name: String, fields: Vec<MLField>) -> Option<&mut MLStruct> {
        self.add_struct(MLStruct { name, fields })
    }

    pub fn add_struct(&mut self, s: MLStruct) -> Option<&mut MLStruct> {
        let name = s.name.clone();
        self.structs.insert(name.clone(), s)?;
        self.get_struct(&name)
    }

    pub fn get_struct(&mut self, name: &String) -> Option<&mut MLStruct> {
        self.structs.get_mut(name)
    }

    pub fn to_mlir_file(self) -> MLFile {
        MLFile {
            name: self.name,
            body: self
                .structs
                .into_values()
                .map(|v| MLDecl::Struct(v))
                .chain(self.variables.into_values().map(|v| MLDecl::Var(v)))
                .chain(self.functions.into_values().map(|v| MLDecl::Fun(v)))
                .collect(),
        }
    }
}
