mod error;
mod fun_builder;

pub use self::error::BuilderError;
pub use self::fun_builder::FunBuilder;
use crate::middle_level_ir::expr::MLExpr;
use crate::middle_level_ir::ml_decl::{MLArgDef, MLDecl, MLField, MLFun, MLStruct, MLVar};
use crate::middle_level_ir::ml_file::MLFile;
use crate::middle_level_ir::ml_type::MLValueType;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MLIRModule {
    functions: HashMap<String, FunBuilder>,
    variables: HashMap<String, MLVar>,
    structs: HashMap<String, MLStruct>,
}

impl MLIRModule {
    pub fn new() -> Self {
        Self {
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
    ) -> Option<&mut FunBuilder> {
        self.add_function(FunBuilder::new(name, args, return_type))
    }

    pub fn add_function(&mut self, fun: FunBuilder) -> Option<&mut FunBuilder> {
        let name = fun.name().clone();
        self.functions.insert(name.clone(), fun)?;
        self.get_function(&name)
    }

    pub fn get_function(&mut self, name: &String) -> Option<&mut FunBuilder> {
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

    pub fn create_global_var(&mut self, name: String, value: MLExpr) -> Option<&mut MLVar> {
        self.add_global_var(MLVar {
            is_mute: false,
            name,
            type_: value.type_(),
            value,
        })
    }

    pub fn add_global_var(&mut self, var: MLVar) -> Option<&mut MLVar> {
        let name = var.name.clone();
        self.variables.insert(name.clone(), var)?;
        self.get_global_var(&name)
    }

    pub fn get_global_var(&mut self, name: &String) -> Option<&mut MLVar> {
        self.variables.get_mut(name)
    }

    pub fn to_mlir_file(self, name: String) -> MLFile {
        MLFile {
            name,
            body: self
                .structs
                .into_iter()
                .map(|(_, v)| MLDecl::Struct(v))
                .chain(self.variables.into_iter().map(|(_, v)| MLDecl::Var(v)))
                .chain(self.functions.into_iter().map(|(_, v)| MLDecl::Fun(v.build())))
                .collect(),
        }
    }

    pub fn create_builder(&mut self) -> MLIRBuilder {
        MLIRBuilder {
            module: self,
            current_function: None,
        }
    }
}

#[derive(Debug)]
pub struct MLIRBuilder<'ctx> {
    module: &'ctx mut MLIRModule,
    current_function: Option<String>,
}

impl<'ctx> MLIRBuilder<'ctx> {
    fn current_function(&mut self) -> Result<&mut FunBuilder, BuilderError> {
        let fun_name = self
            .current_function
            .as_ref()
            .ok_or_else(|| BuilderError::from(format!("Build target not set")))?;
        self.module
            .get_function(fun_name)
            .ok_or_else(|| BuilderError::from(format!("{} is not exist", fun_name)))
    }

    pub fn add_function(&mut self, name: String, args: Vec<MLArgDef>, rtype: MLValueType) {
        self.current_function = Some(name.clone());
        self.module.create_function(name, args, rtype);
    }

    pub fn build_return(&mut self, value: Option<MLExpr>) -> Result<(), BuilderError> {
        let f = self.current_function()?;
        Ok(())
    }
}
