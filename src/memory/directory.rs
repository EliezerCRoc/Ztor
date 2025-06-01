#![allow(warnings)]
use std::collections::HashMap;
use crate::{ast::{DataType, Operator, Value, Context}, VariableValueDirectory, VariableValueTable};

#[derive(Debug)]
pub struct FunctionDirectory {
    pub oFunctions: HashMap<String, FunctionInfo>,
}

impl FunctionDirectory{
    pub fn new() -> Self {
        Self {
            oFunctions: HashMap::new(),
        }
    }

    fn getGlobalVariableIndex(&mut self, _sVarName: String) -> usize{
        let mut _sKeySession = "main".to_string();

        if let Some(oFuncInfo) = self.oFunctions.get_mut(&_sKeySession){               
            //Si main no tiene la variable panic
            if !oFuncInfo.oVariableDirectory.contains_key(&_sVarName) {    
                 panic!("Variable '{}' not exists", _sVarName);                                                       
            }      
            return oFuncInfo.oVariableDirectory[&_sVarName];            
        }  
        else{
            panic!("Function  not found");
        }
    }
    pub fn getParamVector(&mut self, sKeySession:String) -> Vec<String>{
        if let Some(oFuncInfo) = self.oFunctions.get_mut(&sKeySession){   

            return oFuncInfo.oParamVector.clone();            
        }  
        else{
            panic!("Function  not found");
        }
    }

    pub fn getVariableIndex(&mut self, sKeySession:String, _sVarName: String) -> usize{
        
        let mut _sKeySession = sKeySession;
        if _sKeySession == "" || _sKeySession == "main"{

            _sKeySession = "main".to_string();
            return self.getGlobalVariableIndex(_sVarName);
        }
        if let Some(oFuncInfo) = self.oFunctions.get_mut(&_sKeySession){   
            //Si la sesion no tiene la variable validar con global(main)
            if !oFuncInfo.oVariableDirectory.contains_key(&_sVarName) && !oFuncInfo.oParamDirectory.contains_key(&_sVarName)  {                                           
                return self.getGlobalVariableIndex(_sVarName); 
            }      

            if !oFuncInfo.oVariableDirectory.contains_key(&_sVarName) {
                return oFuncInfo.oParamDirectory[&_sVarName];            
            }
            return oFuncInfo.oVariableDirectory[&_sVarName];            
        }  
        else{
            panic!("Function  not found");
        }
    }

    pub fn ImportSession(&mut self,  _sKeySession: String, _oLocalValueTable: VariableValueTable,_oTempValueTable: VariableValueTable){
        if let Some(oFuncInfo) = self.oFunctions.get_mut(&_sKeySession){
            oFuncInfo.ImportSession(_oLocalValueTable, _oTempValueTable);
        }  
        else{
            panic!("Function  not found");
        }
    }

    pub fn setStartPointer(&mut self, key: &str, uIndex: usize){
        if let Some(func_info) = self.oFunctions.get_mut(key) {
            func_info.setStartPointer(uIndex);
           
        } else {
            panic!("Function  not found");
        }
    }
    pub fn getStartPointer(&mut self, key: &str) -> usize{
        if let Some(func_info) = self.oFunctions.get_mut(key) {
            return func_info.getStartPointer();
           
        } else {
            panic!("Function  not found");
        }
    }

    pub fn InsertVariable(&mut self, key: &str, oVariableValueDirectory: &mut VariableValueDirectory, oContext: Context, sName: String, oValue: Value, oType: DataType){
        if let Some(func_info) = self.oFunctions.get_mut(key) {
        
            func_info.InsertVariable(oVariableValueDirectory, oContext, sName, oValue, oType);

           
        } else {
            panic!("Function not found")
        }
    }
}

#[derive(Debug)]
pub struct FunctionInfo {
    pub sReturnType: String,
    pub uStartPointer: usize,
    pub oParamVector: Vec<String>,
    pub oParamDirectory: HashMap<String, usize>, // Tabla de Parametros (Nombre, Espacio Memoria)
    pub oVariableDirectory: HashMap<String, usize>, // Tabla de Variables (Nombre, Espacio Memoria)
    pub oLocalValueTable:  VariableValueTable,
    pub oTempValueTable:  VariableValueTable
}

impl FunctionInfo {
    pub fn new(ReturnType: String, uIndex: usize, _oLocalValueTable: VariableValueTable,_oTempValueTable: VariableValueTable ) -> Self {
        Self {
            sReturnType: ReturnType,            
            uStartPointer: uIndex,
            oParamVector: Vec::new(),
            oParamDirectory: HashMap::new(),
            oVariableDirectory: HashMap::new(),
            oLocalValueTable: _oLocalValueTable,

            oTempValueTable: _oTempValueTable,

        }
    }

    pub fn ImportSession(&mut self, _oLocalValueTable: VariableValueTable,_oTempValueTable: VariableValueTable){
        self.oLocalValueTable = _oLocalValueTable;
        self.oTempValueTable = _oTempValueTable; 
    }

    pub fn getStartPointer(&mut self) -> usize{
        return self.uStartPointer
    }
    pub fn setStartPointer(&mut self, uIndex: usize){
        self.uStartPointer = uIndex;
    }

    pub fn GenerateParam(&mut self,  oVariableValueDirectory: &mut VariableValueDirectory, oContext: Context, sName: String, oType: DataType) -> bool {
        if !self.oParamDirectory.contains_key(&sName) {
            match oVariableValueDirectory.generateVariable(oContext,oType.DefaultValue(), oType) {
                Ok(iIndex) => {

                    self.oParamVector.insert(0,sName.clone());

                    self.oParamDirectory.insert(sName, iIndex);
                    true
                }
                Err(_) => false,
            }
        }
        else {
            false
        }
    }

    pub fn GenerateVariable(&mut self,  oVariableValueDirectory: &mut VariableValueDirectory, oContext: Context, sName: String, oType: DataType) -> bool {

        if (self.oVariableDirectory.get(&sName).is_none()){

            match oVariableValueDirectory.generateVariable(oContext,oType.DefaultValue(), oType) {
                Ok(iIndex) => {
                    self.oVariableDirectory.insert(sName, iIndex);
                    true
                }
                Err(_) => false,
            }
        }
        else {
            false
        }
    }

    pub fn InsertParam(&mut self,  oVariableValueDirectory: &mut VariableValueDirectory, oContext: Context, sName: String, oValue: Value,  oType: DataType) -> bool {
        if (self.oParamDirectory.get(&sName).is_none()){

            match oVariableValueDirectory.generateVariable(oContext,oValue, oType) {
                Ok(iIndex) => {
                    self.oParamDirectory.insert(sName, iIndex);
                    true
                }
                Err(_) => false,
            }
        }
        else {
            false
        }
    }

    pub fn InsertVariable(&mut self,  oVariableValueDirectory: &mut VariableValueDirectory, oContext: Context, sName: String, oValue: Value,  oType: DataType) -> bool {
        if (self.oVariableDirectory.get(&sName).is_none()){

            match oVariableValueDirectory.generateVariable(oContext,oValue, oType) {
                Ok(iIndex) => {
                    self.oVariableDirectory.insert(sName, iIndex);
                    true
                }
                Err(_) => false,
            }
        }
        else {
            false
        }
    }
}