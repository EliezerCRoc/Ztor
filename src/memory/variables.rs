#![allow(warnings)]
use std::{collections::HashMap, ops::Index};
use crate::utils::stack::Stack;
use crate::ast::{DataType, Operator, Value, Expression, Context};



#[derive(Debug)]

pub struct VariableValueTable {    
    iIntCounter: usize,
    iFloatCounter: usize,
    iBoolCounter: usize,
    //iTempCounter: usize,

    oIntVec: Vec<Option<Value>>,
    oFloatVec: Vec<Option<Value>>,
    oBoolVec: Vec<Option<Value>>,


    //oValues: Vec<Option<Value>>,
}



impl VariableValueTable {
    pub fn new() -> Self {
        Self {            
            iIntCounter: (DataType::Int as usize),
            iFloatCounter: (DataType::Float as usize),
            iBoolCounter: (DataType::Bool as usize),

            oIntVec: vec![],
            oFloatVec: vec![],
            oBoolVec: vec![],

            //oValues: vec![None; 10000],


        }
    }

    // Insertar valores Int, Float y Bool en sus respectivas secciones
    pub fn insert<T: Into<Value>>(&mut self, oVal: T, oVariableType: DataType) -> usize {
        let mut iIndex: usize;
        match oVariableType{
            DataType::Int => {iIndex = self.iIntCounter;self.oIntVec.push( Some(oVal.into()));self.iIntCounter += 1; },
            DataType::Float => {iIndex = self.iFloatCounter;self.oFloatVec.push( Some(oVal.into()));self.iFloatCounter += 1;},
            DataType::Bool => { iIndex = self.iBoolCounter;self.oBoolVec.push( Some(oVal.into()));self.iBoolCounter += 1;},
        }                       
        // self.oValues[iIndex] = Some(oVal.into());
        return iIndex;
    }
    
    // Inserta el valor de una variable en su espacio de memoria
    pub fn set(&mut self, uAddress: usize, oVal: Value) {   
        let mut _uAddress:usize = uAddress;                  
        match DataType::GetType(_uAddress){
            DataType::Int=> {_uAddress = _uAddress - (DataType::Int as usize);self.oIntVec[_uAddress] = Some(oVal); },
            DataType::Float => {_uAddress = _uAddress - (DataType::Float as usize); self.oFloatVec[_uAddress] = Some(oVal);},
            DataType::Bool => {_uAddress = _uAddress - (DataType::Bool as usize);  self.oBoolVec[_uAddress] = Some(oVal);},
        }
        //self.oValues[uAddress] = Some(oVal);
    }
    pub fn get(&self, uAddress: usize) -> Option<&Value> {
        let mut _uAddress:usize = uAddress;                  

        match DataType::GetType(_uAddress){
            DataType::Int => {_uAddress = _uAddress - (DataType::Int as usize);self.oIntVec.get(_uAddress)?.as_ref() },
            DataType::Float => {_uAddress = _uAddress - (DataType::Float as usize);self.oFloatVec.get(_uAddress)?.as_ref()},
            DataType::Bool => {_uAddress = _uAddress - (DataType::Bool as usize);self.oBoolVec.get(_uAddress)?.as_ref()},
        }
        //self.oValues.get(addr)?.as_ref() // Con el ? regresa None en los casos que no exista
    }


}

#[derive(Debug)]
pub struct VariableValueDirectory {
    uGlobalOffsize: usize,
    uConstantOffsize: usize,
    uLocalOffsize:usize,
    uTempOffsize: usize,

    // Tablas globales
    oGlobalTable: VariableValueTable,
    oConstantTable: VariableValueTable,

    // Cuando se generen diferentes sesiones(llamadas a funciones), se guardara en stack la sesion y se generara uno nuevo
    oLocalDirectory: Stack<VariableValueTable>,
    oTempDirectory: Stack<VariableValueTable>,

    // Tablas actuales de sesion
    oLocalValueTable: VariableValueTable,
    oTempValueTable: VariableValueTable, // Siempre inicia con uno principal refiriendose a la tabla Global
}

impl VariableValueDirectory{
    pub fn new() -> Self{
        Self {
            uGlobalOffsize:(Context::Global as usize),
            uConstantOffsize:(Context::Constant as usize),
            uLocalOffsize:(Context::Local as usize),
            uTempOffsize:(Context::Temp as usize),

            oGlobalTable: VariableValueTable::new(),
            oConstantTable: VariableValueTable::new(),

            oLocalDirectory: Stack::new(),
            oTempDirectory: Stack::new(),

            oLocalValueTable: VariableValueTable::new(),
            oTempValueTable: VariableValueTable::new()

        }
    }

    fn getTableType(&mut self, uIndex: usize) -> Context{
        
        let mut oContext: Context = Context::Global;

        if uIndex >= Context::Constant as usize && uIndex < Context::Local as usize {
            oContext = Context::Constant;

        } else if uIndex >= Context::Local as usize && uIndex < Context::Temp as usize {
            oContext = Context::Local;
        } else if uIndex >= Context::Temp as usize {
            oContext = Context::Temp;
        } 
        
        return oContext;
    }

    pub fn generateVariable(&mut self, oContext: Context, oValue: Value, oType: DataType) -> Result<usize, String>{
 
        match oContext {
            Context::Global => {
                return Ok(self.oGlobalTable.insert(oValue, oType) + self.uGlobalOffsize);
            },
            Context::Constant => {
                return Ok(self.oConstantTable.insert(oValue, oType) + self.uConstantOffsize);
            },
            Context::Local => {
                return Ok(self.oLocalValueTable.insert(oValue, oType) + self.uLocalOffsize);
            },
            Context::Temp => {
                return Ok(self.oTempValueTable.insert(oValue, oType) + self.uTempOffsize);
            },
            _ => {
                panic!("Error: Generating Value in Memory");
            }
        }
    }

    pub fn setValue(&mut self, uIndex: &mut usize, oValue: Value) {
                   


        match self.getTableType(*uIndex) {
            Context::Global => {
                *uIndex = *uIndex - self.uGlobalOffsize;
                self.oGlobalTable.set(*uIndex,oValue);
            },
            Context::Constant => {
                *uIndex = *uIndex - self.uConstantOffsize;
                self.oConstantTable.set(*uIndex,oValue);
            },
            Context::Local => {
                *uIndex = *uIndex - self.uLocalOffsize;
                self.oLocalValueTable.set(*uIndex,oValue);
            },
            Context::Temp => {
                *uIndex = *uIndex - self.uTempOffsize;                
                self.oTempValueTable.set(*uIndex,oValue);
            },
            _ => {
                panic!("Error: Inserting Value in Memory");
            }
        }

    }
    
    pub fn getValue(&mut self, uIndex: &mut usize) -> Option<&Value> {
        let oResult: Option<&Value>;


        match self.getTableType(*uIndex) {
            Context::Global => {

                *uIndex = *uIndex - self.uGlobalOffsize;                
                oResult = self.oGlobalTable.get(*uIndex);
            },
            Context::Constant => {
                *uIndex = *uIndex - self.uConstantOffsize;
                oResult = self.oConstantTable.get(*uIndex);
            },
            Context::Local => {
                *uIndex = *uIndex - self.uLocalOffsize;
                oResult = self.oLocalValueTable.get(*uIndex);
            },
            Context::Temp => {
                *uIndex = *uIndex - self.uTempOffsize;
                oResult = self.oTempValueTable.get(*uIndex);
            },
            _ => {
                panic!("Error:Getting Value from Memory");
            }

        }
        return oResult;
    }
    // // Obtener session actual, saber si esta en Global, Local
    // pub fn GetActualSession(&mut self) -> VariableValueTable{
    //     // Si no se tiene nada en el directorio local es porque ya estamos en main
    //     match self.oLocalDirectory.pop() {
    //         Some(oVariableValueTable) => {return oVariableValueTable},
    //         None => {return self.oGlobalTable}
    //     };

    // }
}