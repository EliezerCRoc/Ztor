#![allow(warnings)]
use std::collections::HashMap;
use crate::{ast::{DataType, Operator, Value, Context}, VariableValueDirectory, VariableValueTable};

#[derive(Debug)]
pub struct FunctionDirectory {
    pub oFunctions: HashMap<String, FunctionInfo>,
}

impl FunctionDirectory {
    pub fn new() -> Self {
        Self {
            oFunctions: HashMap::new(),
        }
    }

    pub fn setStartPointer(&mut self, key: &str, uIndex: usize){
        if let Some(func_info) = self.oFunctions.get_mut(key) {
            func_info.setStartPointer(uIndex);
           
        } else {
            panic!("Function  not found");
        }
    }

    pub fn InsertVariable(&mut self, key: &str, oVariableValueDirectory: &mut VariableValueDirectory, oContext: Context, sName: String, oValue: Value, oType: DataType){
        if let Some(func_info) = self.oFunctions.get_mut(key) {
            if (self.oFunctions[key].oVariableDirectory.get(&sName).is_none()){
            //Genera la Constante en tabla de constante, checar variables.rs para ver que numero es de cada contexto
                match oVariableValueDirectory.generateVariable(Context::Constant, oValue, DataType::Int) {
                        Ok(iIndex) => {     


                            self
                            .oFunctions
                            .get_mut("main")
                            .unwrap()
                            .oVariableDirectory
                            .insert(sName, iIndex);
                            
                        
                        },
                        Err(e) => {
                            panic!("Error al insertar en tabla de variables: {}", e);
                        }
                    }
                
            }
           
        } else {
            panic!("Function not found")
        }
    }
}

#[derive(Debug)]
pub struct FunctionInfo {
    pub sReturnType: String,
    pub uStartPointer: usize,
    pub oVariableDirectory: HashMap<String, usize>, // Tabla de Variables (Nombre, Espacio Memoria)
    pub oLocalValueTable: VariableValueTable,
    pub oTempValueTable: VariableValueTable
}

impl FunctionInfo {
    pub fn new(ReturnType: String, uIndex: usize ) -> Self {
        Self {
            sReturnType: ReturnType,            
            uStartPointer: uIndex,
            oVariableDirectory: HashMap::new(),
            oLocalValueTable: VariableValueTable::new(),
            oTempValueTable: VariableValueTable::new(),

        }
    }
    pub fn setStartPointer(&mut self, uIndex: usize){
        self.uStartPointer = uIndex;
    }

    pub fn InsertVariable(&mut self,  oVariableValueDirectory: &mut VariableValueDirectory, oContext: Context, sName: String, oType: DataType) -> bool {
        match oVariableValueDirectory.generateVariable(oContext,oType.DefaultValue(), oType) {
            Ok(iIndex) => {
                self.oVariableDirectory.insert(sName, iIndex);
                true
            }
            Err(_) => false,
        }
    }
}