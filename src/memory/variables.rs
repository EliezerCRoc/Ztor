#![allow(warnings)]
use std::{collections::HashMap, ops::Index};
use crate::utils::stack::Stack;
use crate::ast::{DataType, Operator, Value, Expression, Context};



#[derive(Debug, Clone)]

pub struct VariableValueTable {    
    iIntCounter: usize,
    iFloatCounter: usize,
    iBoolCounter: usize,
    iStringCounter: usize,
    //iTempCounter: usize,

    oIntVec: Vec<Option<Value>>,
    oFloatVec: Vec<Option<Value>>,
    oBoolVec: Vec<Option<Value>>,
    oStringVec: Vec<Option<Value>>,



    //oValues: Vec<Option<Value>>,
}



impl VariableValueTable {
    pub fn new() -> Self {
        Self {            
            iIntCounter: (DataType::Int as usize),
            iFloatCounter: (DataType::Float as usize),
            iBoolCounter: (DataType::Bool as usize),
            iStringCounter: (DataType::String as usize),


            oIntVec: vec![],
            oFloatVec: vec![],
            oBoolVec: vec![],
            oStringVec: vec![],


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
            DataType::String => { iIndex = self.iStringCounter;self.oStringVec.push( Some(oVal.into()));self.iStringCounter += 1;},

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
            DataType::String => {_uAddress = _uAddress - (DataType::String as usize);  self.oStringVec[_uAddress] = Some(oVal);},

        }
        //self.oValues[uAddress] = Some(oVal);
    }
    pub fn get(&self, uAddress: usize) -> Option<&Value> {
        let mut _uAddress:usize = uAddress;                  
        
        match DataType::GetType(_uAddress){
            DataType::Int => {_uAddress = _uAddress - (DataType::Int as usize);self.oIntVec.get(_uAddress)?.as_ref() },
            DataType::Float => {_uAddress = _uAddress - (DataType::Float as usize);self.oFloatVec.get(_uAddress)?.as_ref()},
            DataType::Bool => {_uAddress = _uAddress - (DataType::Bool as usize);self.oBoolVec.get(_uAddress)?.as_ref()},
            DataType::String => {_uAddress = _uAddress - (DataType::String as usize);self.oStringVec.get(_uAddress)?.as_ref()},

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

    // Tablas de anterior sesion
    oLocalValueTableTemp: VariableValueTable,
    oTempValueTableTemp: VariableValueTable,


    // Tablas actuales de sesion
    oLocalValueTable: VariableValueTable,
    oTempValueTable: VariableValueTable, // Siempre inicia con uno principal refiriendose a la tabla Global

    bUseSession: bool,
    sKeySession: String,
    //Esta variable solo se usa cuando se estan asignando parametros en la llamada de una funcion
    bUseParameterTemp: bool

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

            oLocalValueTableTemp: VariableValueTable::new(),
            oTempValueTableTemp: VariableValueTable::new(),


            oLocalValueTable: VariableValueTable::new(),
            oTempValueTable: VariableValueTable::new(),

            //Como iniciamos en main no requerimos el uso de sesion
            bUseSession: false,
            sKeySession: String::new(),
            bUseParameterTemp: false
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
        
        let mut _oContext = oContext;
        //Si la sesion esta activa y se quiere guardar en global, se pasa a local
        // Global -> Local
        if(self.bUseSession && _oContext == Context::Global){
            _oContext = Context::Local;
        }

        match _oContext {
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
                if self.bUseParameterTemp{
                    self.oLocalValueTableTemp.set(*uIndex,oValue);
                    return;
                }
                self.oLocalValueTable.set(*uIndex,oValue);
            },
            Context::Temp => {
                *uIndex = *uIndex - self.uTempOffsize;                
                if self.bUseParameterTemp{
                    self.oTempValueTableTemp.set(*uIndex,oValue);
                    return;
                }
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
                if self.bUseParameterTemp{
                    oResult = self.oLocalValueTableTemp.get(*uIndex);
                    return oResult;
                }
                oResult = self.oLocalValueTable.get(*uIndex);
            },
            Context::Temp => {
                *uIndex = *uIndex - self.uTempOffsize;
                if self.bUseParameterTemp{
                    oResult = self.oTempValueTable.get(*uIndex);
                    return oResult;
                }
                oResult = self.oTempValueTable.get(*uIndex);
            },
            _ => {
                panic!("Error:Getting Value from Memory");
            }

        }
        return oResult;
    }
    
    //Obtener valores de Sesion Anterior antes de ser guardado en Stack
    pub fn getValueLastSession(&mut self, uIndex: &mut usize) -> Option<&Value> {

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
                oResult = self.oLocalValueTableTemp.get(*uIndex);
            },
            Context::Temp => {
                *uIndex = *uIndex - self.uTempOffsize;
                oResult = self.oTempValueTableTemp.get(*uIndex);
            },
            _ => {
                panic!("Error:Getting Value from Memory");
            }

        }
        return oResult;
    }

    pub fn clearTemp(&mut self){
        self.oLocalValueTableTemp = VariableValueTable::new();
        self.oTempValueTableTemp = VariableValueTable::new();
    }

    pub fn SetKeySession(&mut self, _sKeySession: String){
        self.sKeySession = _sKeySession;
    }

    pub fn GetKeySession(&mut self) -> String{
        return self.sKeySession.clone();
    }
    pub fn setUseSession(&mut self){
        self.bUseSession = !self.bUseSession;
    }

    pub fn setUseParameter(&mut self){
        self.bUseParameterTemp = !self.bUseParameterTemp;
    }
    
    //Genera una nueva sesion
    pub fn GenerateSession(&mut self){
        if(!self.bUseSession) {            
            self.setUseSession();
        }
        self.SaveSessionStack();
    }

    // Guarda la sesion que se estuviera usando actualmente en el stack 
    pub fn SaveSessionStack(&mut self){
        self.oLocalDirectory.push(self.oLocalValueTable.clone());
        self.oTempDirectory.push(self.oTempValueTable.clone());

        self.oLocalValueTableTemp = self.oLocalValueTable.clone();
        self.oTempValueTableTemp = self.oTempValueTable.clone();

        self.oLocalValueTable = VariableValueTable::new();
        self.oTempValueTable = VariableValueTable::new();
    }

    //Obtener la sesion que se haya guardado en el stack
    pub fn GetSessionStack(&mut self){
        match self.oLocalDirectory.pop() {
            Some(_oLocalTable) => {

                    self.oLocalValueTable =_oLocalTable ;

            }
            _ => ()
        }

        match self.oTempDirectory.pop() {
            Some(_oTempValueTable) => {
                self.oTempValueTable =_oTempValueTable ;
            }
            _ => ()
        }
    }

    // Importar sesion gnerada del directorio de funciones 
    pub fn ImportSession(&mut self, _oLocalValueTable: VariableValueTable, _oTempValueTable: VariableValueTable) {
        self.oLocalValueTable = _oLocalValueTable;
        self.oTempValueTable = _oTempValueTable;
    }

    // Exportar sesion (normalmente se usa cuando recien se va a generar la funcion)    
    pub fn ExportSession(&mut self, bOption: bool) -> VariableValueTable{
        if(bOption) {
            return self.oLocalValueTable.clone();
        }
        else {
            return self.oTempValueTable.clone();
        }
    }
}