#![allow(warnings)]
use crate::ast::{Value, Operator, Context};
use crate::memory::{variables::{VariableValueDirectory}, directory::FunctionDirectory};
use crate::semantic::quadruples::{Quadruple, QuadrupleList};
use crate::utils::stack::Stack;
use crate::VariableValueTable;



pub struct Executer<'a>{
    oFunctionDirectory: &'a mut FunctionDirectory,
    oVariableValueDirectory: &'a mut VariableValueDirectory,
    oQuadrupleList: &'a mut QuadrupleList,
}

impl<'a> Executer<'a>{
    pub fn new( _oFunctionDirectory: &'a mut FunctionDirectory, 
            _oVariableValueDirectory: &'a mut VariableValueDirectory,
            _oQuadrupleList: &'a mut QuadrupleList,) -> Self {
        Self {
            oFunctionDirectory: _oFunctionDirectory,
            oVariableValueDirectory: _oVariableValueDirectory,
            oQuadrupleList: _oQuadrupleList
        }
    }

    pub fn executeQuadruple(&mut self){
        let mut oCounterStack: Stack<usize> = Stack::new() ;
        let mut  uCounter:usize = 0;        
        while uCounter < self.oQuadrupleList.getSize() {
            let oQuadruple:&Quadruple = &self.oQuadrupleList.getQuadruple(uCounter);
            uCounter = uCounter + 1;

            match oQuadruple.iOperator{
                Operator::Add   |   Operator::Sub |
                Operator::Mul   |   Operator::Div =>{
                    let mut oArg1 =  oQuadruple.oArg1.expect("Missing Arg 1");
                    let mut oArg2 =  oQuadruple.oArg2.expect("Missing Arg 2"); 
                    let mut oResultAddress =  oQuadruple.oResult.expect("Missing Result address");                     
                    
                    let oArgValue1 = self.oVariableValueDirectory.getValue( &mut oArg1).unwrap().clone();
              

                    let oArgValue2 = self.oVariableValueDirectory.getValue(&mut oArg2).unwrap().clone();

                    let oResult = match (oArgValue1, oArgValue2) {
                        (Value::Int(i1), Value::Int(i2)) => {
                            let r = match oQuadruple.iOperator {
                                Operator::Add => i1 + i2,
                                Operator::Sub => i1 - i2,
                                Operator::Mul => i1 * i2,
                                Operator::Div => i1 / i2,
                                _ => unreachable!(),
                            };
                            Value::Int(r)
                        },
                        (Value::Float(f1), Value::Float(f2)) => {
                            let r = match oQuadruple.iOperator {
                                Operator::Add => f1 + f2,
                                Operator::Sub => f1 - f2,
                                Operator::Mul => f1 * f2,
                                Operator::Div => f1 / f2,
                                _ => unreachable!(),
                            };
                            Value::Float(r)
                        },
                        (Value::Int(i1), Value::Float(f2)) => {
                            let r = match oQuadruple.iOperator {
                                Operator::Add => (i1 as f64) + f2,
                                Operator::Sub => (i1 as f64) - f2,
                                Operator::Mul => (i1 as f64) * f2,
                                Operator::Div => (i1 as f64) / f2,
                                _ => unreachable!(),
                            };
                            Value::Float(r)
                        },
                        (Value::Float(f1), Value::Int(i2)) => {
                            let r = match oQuadruple.iOperator {
                                Operator::Add => f1 + (i2 as f64),
                                Operator::Sub => f1 - (i2 as f64),
                                Operator::Mul => f1 * (i2 as f64),
                                Operator::Div => f1 / (i2 as f64),
                                _ => unreachable!(),
                            };
                            Value::Float(r)
                        },
                        _ => panic!("Type mismatch in arithmetic operation"),
                    };
                    self.oVariableValueDirectory.setValue(&mut oResultAddress, oResult);

                }
                Operator::GreaterThan   |   Operator::NotEqual |
                Operator::LessThan   =>{
                    let mut oArg1 =  oQuadruple.oArg1.expect("Missing Arg 1");
                    let mut oArg2 =  oQuadruple.oArg2.expect("Missing Arg 2"); 
                    let mut oResultAddress =  oQuadruple.oResult.expect("Missing Result address"); 
                    
                    let oArgValue1 = self.oVariableValueDirectory.getValue( &mut oArg1).unwrap().clone();
              

                    let oArgValue2 = self.oVariableValueDirectory.getValue(&mut oArg2).unwrap().clone();

                    let oResult = match (oArgValue1, oArgValue2) {
                        (Value::Int(i1), Value::Int(i2)) => {
                            let r = match oQuadruple.iOperator {
                                Operator::GreaterThan => i1 > i2,
                                Operator::NotEqual => i1 != i2,
                                Operator::LessThan => i1 < i2,
                                
                                _ => unreachable!(),
                            };
                            Value::Bool(r)
                        },
                        (Value::Float(f1), Value::Float(f2)) => {
                            let r = match oQuadruple.iOperator {
                                Operator::GreaterThan => f1 > f2,
                                Operator::NotEqual => f1 != f2,
                                Operator::LessThan => f1 < f2,
                                _ => unreachable!(),
                            };
                            Value::Bool(r)
                        },
                        _ => panic!("Type mismatch in arithmetic operation"),
                    };
                    self.oVariableValueDirectory.setValue(&mut oResultAddress, oResult);
                }
                Operator::Print => {
                    
                    let mut oResultAddress =  oQuadruple.oResult.expect("Missing Result address");                                        
                    let oResultValue = self.oVariableValueDirectory.getValue(&mut oResultAddress).unwrap().clone();                    

                    match oResultValue {
                        Value::Int(oValue) => println!("{}", oValue),
                        Value::Float(oValue) => println!("{}", oValue),
                        Value::Bool(oValue) => println!("{}",oValue),
                        Value::String(oValue) => println!("{}", oValue),
                        _ => println!()
                    }
                    //println!("{:?}",oResultValue);
                }
                Operator::Assign => {
                    let mut oValueAddress =  oQuadruple.oArg1.expect("Missing Value Assign address");                    
                    let mut oVariableResult =  oQuadruple.oResult.expect("Missing Variable address");
                    let oValue = self.oVariableValueDirectory.getValue( &mut oValueAddress).unwrap().clone();                               
                                     
                    self.oVariableValueDirectory.setValue(&mut oVariableResult, oValue);       

                }
                Operator::GotoF => {
                    let mut oArg1Address =  oQuadruple.oArg1.expect("Missing Value address");                    
                    let mut uNewCounter =  oQuadruple.oResult.expect("Missing Variable address");
                    let oValueBool = self.oVariableValueDirectory.getValue( &mut oArg1Address).unwrap().clone();
                    
                    if(oValueBool == Value::Bool(false)){
                        uCounter = uNewCounter;
                    }                    
                }
                Operator::Goto => {                    
                    let mut uNewCounter =  oQuadruple.oResult.expect("Missing Variable address");                    
                
                    uCounter = uNewCounter;                    
                }
                Operator::Era => {
                    let mut uFunctionRedirectAddress =  oQuadruple.oResult.expect("Missing Function address");                    
                    let oFunctionRedirectValue = self.oVariableValueDirectory.getValue( &mut uFunctionRedirectAddress).unwrap().clone();
                    
                    let sFunction: String = match(oFunctionRedirectValue){
                        Value::String(sFunctionName) => sFunctionName,
                        _ => {panic!("Error Function not found in index");}
                    } ;


                    self.oVariableValueDirectory.SaveSessionStack();
                    if let Some(oFunctionInfo) = self.oFunctionDirectory.oFunctions.get_mut(&sFunction){
                        self.oVariableValueDirectory.SetKeySession(sFunction);
                        self.oVariableValueDirectory.ImportSession(oFunctionInfo.oLocalValueTable.clone(), oFunctionInfo.oTempValueTable.clone());
                        self.oVariableValueDirectory.setUseSession();       
                        self.oVariableValueDirectory.setUseParameter();                


                    }
                }
                Operator::Param => {
                    let mut oValueAddress =  oQuadruple.oArg1.expect("Missing Value Assign address");                    
                    let oValue = self.oVariableValueDirectory.getValueLastSession( &mut oValueAddress).unwrap().clone();  

                    let mut oVariableResult =  oQuadruple.oResult.expect("Missing Variable address");
                    
                    self.oVariableValueDirectory.setUseParameter();      
                    self.oVariableValueDirectory.setValue(&mut oVariableResult, oValue);      
                    self.oVariableValueDirectory.setUseParameter();      

                    
                }
                Operator::GoSub => {
                    

                    let mut uFunctionIndex: usize;

                    self.oVariableValueDirectory.clearTemp();      
                    self.oVariableValueDirectory.setUseParameter();      

                    
                    if let Some(oFunctionInfo) = self.oFunctionDirectory.oFunctions.get_mut(&self.oVariableValueDirectory.GetKeySession()){
                        uFunctionIndex = oFunctionInfo.getStartPointer();
                        oCounterStack.push(uCounter);
                        uCounter = uFunctionIndex;   
                    }
                    else {
                        panic!("Function Not Found");
                    }

                        
                }
                Operator::FinishFunction => {                    
                    //Recuperamos la sesion anterior
                    self.oVariableValueDirectory.GetSessionStack();
                    uCounter = oCounterStack.pop().unwrap().clone();                      
                }
                _ => {

                }
            }

        }

    }
}