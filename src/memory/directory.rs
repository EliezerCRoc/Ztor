use std::collections::HashMap;
use crate::ast::{DataType, Operator, Value, Expression};

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
}

#[derive(Debug)]
pub struct FunctionInfo {
    pub sReturnType: String,
    pub oVariableDirectory: HashMap<String, usize>, // Tabla de Variables (Nombre, Espacio Memoria)
}

impl FunctionInfo {
    pub fn new(ReturnType: String) -> Self {
        Self {
            sReturnType: ReturnType,            
            oVariableDirectory: HashMap::new()
        }
    }
}