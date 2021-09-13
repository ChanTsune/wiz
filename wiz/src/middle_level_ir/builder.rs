use crate::middle_level_ir::ml_decl::{MLArgDef, MLDecl, MLFun, MLStruct, MLVar};
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::ml_type::MLType;
use std::collections::HashMap;

pub struct MLIRModule {
    name: String,
    functions: HashMap<String, MLFun>,
    variables: HashMap<String, MLVar>,
    structs: HashMap<String, MLStruct>,
}

impl MLIRModule {
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
        return_type: MLType,
    ) -> Option<&mut MLFun> {
        self.functions.insert(
            name.clone(),
            MLFun {
                modifiers: vec![],
                name: name.clone(),
                arg_defs: args,
                return_type,
                body: None,
            },
        )?;
        self.functions.get_mut(&*name)
    }

    pub fn add_function(&mut self, fun: MLFun) -> Option<&mut MLFun> {
        let name = fun.name.clone();
        self.functions.insert(name.clone(), fun)?;
        self.functions.get_mut(&*name)
    }

    pub fn get_function(&mut self, name: &String) -> Option<&mut MLFun> {
        self.functions.get_mut(name)
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
