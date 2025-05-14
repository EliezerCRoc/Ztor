use std::collections::HashMap;

use crate::semantic::datatype::{DataType, Value};


pub struct VariableValueTable {    
    iIntCounter: usize,
    iFloatCounter: usize,
    iBoolCounter: usize,
    oValues: Vec<Option<Value>>,
}

impl VariableValueTable {
    pub fn new() -> Self {
        Self {            
            iIntCounter: 1000,
            iFloatCounter: 2000,
            iBoolCounter: 3000,
            oValues: vec![None; 5000],
        }
    }

    pub fn finish_globalvariables(&mut self) {
        self.iIntCounter += 100;
        self.iFloatCounter += 100;
        self.iBoolCounter += 100;        
    }

    pub fn set(&mut self, oVal: Value, oVariableType: DataType) -> usize {
        let mut iIndex: usize;
        match oVariableType{
            DataType::Int => {self.iIntCounter += 1;iIndex = self.iIntCounter },
            DataType::Float => {self.iFloatCounter += 1;iIndex = self.iFloatCounter},
            DataType::Bool => {self.iBoolCounter += 1; iIndex = self.iBoolCounter},
        }                       
        self.oValues[iIndex] = Some(oVal);
        iIndex
        
    }

    pub fn get(&self, addr: usize) -> Option<&Value> {
        self.oValues.get(addr)?.as_ref() // Con el ? regresa None en los casos que no exista
    }
}
