use std::collections::HashMap;

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
    pub oVariableDirectory: HashMap<String, String>, // Tabla de Variables (Nombre, Tipo)
}

impl FunctionInfo {
    pub fn new(ReturnType: String) -> Self {
        Self {
            sReturnType: ReturnType,            
            oVariableDirectory: HashMap::new()
        }
    }
}