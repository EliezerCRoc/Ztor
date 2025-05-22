#![allow(warnings)]
use std::{collections::HashMap, ops::Index};
use crate::ast::{DataType, Operator, Value, Expression};

#[derive(Debug)]

pub struct VariableValueTable {    
    iIntCounter: usize,
    iFloatCounter: usize,
    iBoolCounter: usize,
    //iTempCounter: usize,

    oValues: Vec<Option<Value>>,
}

impl VariableValueTable {
    pub fn new() -> Self {
        Self {            
            iIntCounter: 1000,
            iFloatCounter: 2000,
            iBoolCounter: 3000,
            //iTempCounter: 4000,
            oValues: vec![None; 7000],
        }
    }

    // Insertar valores Int, Float y Bool en sus respectivas secciones
    pub fn insert<T: Into<Value>>(&mut self, oVal: T, oVariableType: DataType) -> Result<usize, String> {
        let mut iIndex: usize;
        match oVariableType{
            DataType::Int => {self.iIntCounter += 1;iIndex = self.iIntCounter },
            DataType::Float => {self.iFloatCounter += 1;iIndex = self.iFloatCounter},
            DataType::Bool => {self.iBoolCounter += 1; iIndex = self.iBoolCounter},
        }                       
        self.oValues[iIndex] = Some(oVal.into());
        Ok(iIndex)        
    }
    // Insertar valores en temp (principalmente para los temporales al generar expresiones)
    // pub fn insertTemp(&mut self, oVal: Value) -> usize {
    //     self.iTempCounter += 1;
    //     let mut iIndex: usize = self.iTempCounter;               
    //     self.oValues[iIndex] = Some(oVal);
    //     iIndex        
    // }
    
    // Inserta el valor de una variable en su espacio de memoria
    pub fn set(&mut self, oVal: Value, uAddress: usize) {           
        self.oValues[uAddress] = Some(oVal);
    }
    pub fn get(&self, addr: usize) -> Option<&Value> {
        self.oValues.get(addr)?.as_ref() // Con el ? regresa None en los casos que no exista
    }


}
