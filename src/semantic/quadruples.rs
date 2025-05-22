#![allow(warnings)]
use std::collections::HashMap;

use crate::semantic::cube::SemanticCube;
use crate::ast::{DataType, Operator, Value, Expression};

use crate::utils::stack::Stack;
use crate::memory::variables::VariableValueTable;

#[derive(Debug)]
enum PolacItem {
    Operator(Operator), 
    Address(usize),     
}
#[derive(Debug)]
pub struct Quadruple{
    pub iOperator: Operator,
    //Guarda la direccion de memoria de los argumentos
    pub oArg1: Option<usize>, 
    pub oArg2: Option<usize>,
    pub oResult: Option<usize>
}

impl Quadruple{
    pub fn new(_iOperator: Operator, _oArg1: Option<usize>, _oArg2: Option<usize>, _oResult: Option<usize>) -> Self {
        Self { 
            iOperator: _iOperator, 
            oArg1: _oArg1, 
            oArg2: _oArg2, 
            oResult: _oResult
        }
    }     
}

#[derive(Debug)]
pub struct QuadrupleList {
    //Tabla de Variables parecida a la directory para las funciones, pero aqui solo guarda temporales
    //Usa la misma tabla de variables de memoria, y tambien se usa para guardar las constantes
    pub oVariableTempTable: HashMap<String, usize>, // El usize minimo es el que esta en Variables: 4000

    //Stack de operadores
    pub oOperatorStack: Stack<Operator>,

    //Stack de Variables y Constantes, solo guarda la memoria
    pub oOperandStack: Stack<usize>,

    //Stack de Variables y Constantes, solo guarda la memoria
    pub oJumpsStack: Stack<usize>,

    //Vector de cuadruplos, aqui se registraran todas las acciones
    pub oQuadruples: Vec<Quadruple>,
}

impl QuadrupleList {

    pub fn new() -> Self {
        Self {
            oVariableTempTable: HashMap::new(),
            oOperatorStack: Stack::new(),
            oOperandStack: Stack::new(),    
            oJumpsStack: Stack::new(),
            oQuadruples: Vec::new(),
        }
    }

    pub fn InsertOperand(&mut self, _uVar: usize){
        self.oOperandStack.push(_uVar);
    }

    pub fn InsertOperator(&mut self, _oOperator: Operator){
        self.oOperatorStack.push(_oOperator);
    }

    pub fn InsertJump(&mut self, iQuadrupleIndex: usize){
        
        self.oJumpsStack.push(iQuadrupleIndex);
    }

    pub fn DeleteOperator(&mut self){
        self.oOperatorStack.pop();
    }

    pub fn getType(&self, addr: usize) -> DataType {
        if(addr < 2000 && addr > 1000){
            DataType::Int
        }
        else if (addr < 3000 && addr > 2000){
            DataType::Float
        }
        else {
            DataType::Bool
        }
    }

    pub fn CheckSemanticCube(&mut self,uRighOperand: usize, uLeftOperand: usize, _oOperator: Operator) -> Option<DataType>{
        let oSemanticCube = SemanticCube::new();

        let mut oRightOperandType: DataType = self.getType(uRighOperand);
        let mut oLeftOperandType: DataType = self.getType(uLeftOperand);
        
        oSemanticCube.result_type(oLeftOperandType, oRightOperandType, _oOperator)

    }
    // Genera Cuadruplo para todos los operadores
    // Regresa Bool: True: Si funciono | False : No Funciono
     pub fn GenerateQuadruple(&mut self, oVariableValueTable: &mut VariableValueTable) -> bool{
        if let Some(operator_ref) = self.oOperatorStack.peek() {
            match operator_ref {
                Operator::Add | Operator::Sub |
                Operator::Mul | Operator::Div | 
                Operator::GreaterThan | Operator::LessThan |
                Operator::NotEqual  => {
                   
                    let right_operand = self.oOperandStack.pop().unwrap();
                    let operator = self.oOperatorStack.pop().unwrap();
                    let left_operand = self.oOperandStack.pop().unwrap();                     
                    let result_type = self.CheckSemanticCube(right_operand, left_operand, operator);
                    if let Some(r_type) = result_type {
                        match oVariableValueTable.insert(r_type.DefaultValue(), r_type) { // Aqui se insertara el valor del resultado de las expresiones
                        Ok(result_address) => {
                            let quad = Quadruple::new(operator, 
                                                            Some(left_operand),
                                                            Some(right_operand), 
                                                            Some(result_address));
                            self.oQuadruples.push(quad);

                            self.oOperandStack.push(result_address);
                            return true;
                        }
                        Err(_) => {return false;},
                    }
                        
                    } else {
                        panic!("Type mismatch: {:?} {:?} {:?}", left_operand, operator, right_operand);
                        return false;
                    }
                }       
                Operator::Assign => {
                    let oVariable = self.oOperandStack.pop();

                    let oOperator = self.oOperatorStack.pop().unwrap();
                    let oResult = self.oOperandStack.pop();//self.oQuadruples.last().unwrap().oResult;
                    
                    let oQuadrupleTemp = Quadruple::new(oOperator, 
                                                                    oVariable, 
                                                                    None,
                                                                     oResult
                                                                );
                    self.oQuadruples.push(oQuadrupleTemp);
                }   
                  
                _ => { // Para cualquier otro caso
                    return false;
                }
            }
        }
        false 
    }

    pub fn GenerateQuadrupleConditional(&mut self, oVariableValueTable: &mut VariableValueTable) -> bool{
        if let Some(operator_ref) = self.oOperatorStack.peek() {
            match operator_ref {                
                Operator::GotoF => {
                    
                    let oOperator = self.oOperatorStack.pop().unwrap();
                    let oResult = self.oOperandStack.pop();//self.oQuadruples.last().unwrap().oResult;
                    
                    let oQuadrupleTemp = Quadruple::new(oOperator, 
                                                                    oResult, 
                                                                    None,
                                                                     None
                                                                );
                    self.oQuadruples.push(oQuadrupleTemp);
                    // Obtener el indice del quadruplo que ocupa saber el salto siguiente
                    let iQuadrupleIndex = self.oQuadruples.len().checked_sub(1); 
                    self.InsertJump(iQuadrupleIndex.unwrap());                                         
                } ,
                Operator::Goto => {
                    let oOperator = self.oOperatorStack.pop().unwrap();                    
                    
                    let oQuadrupleTemp = Quadruple::new(oOperator, 
                                                                    None, 
                                                                    None,
                                                                     None
                                                                );
                    self.oQuadruples.push(oQuadrupleTemp);                    
                    self.FinishJump();
                    // Obtener el indice del quadruplo que ocupa saber el salto siguiente
                    let iQuadrupleIndex = self.oQuadruples.len().checked_sub(1); 
                    self.oJumpsStack.push(iQuadrupleIndex.unwrap());   
                }
   
                _ => { // Para cualquier otro caso
                    return false;
                }
            }
        }
        false 
    }

    pub fn GenerateQuadrupleCycle(&mut self, oVariableValueTable: &mut VariableValueTable) -> bool{
        if let Some(operator_ref) = self.oOperatorStack.peek() {
            match operator_ref {                
                Operator::GotoF => {
                    
                    let oOperator = self.oOperatorStack.pop().unwrap();
                    let oResult = self.oOperandStack.pop();//self.oQuadruples.last().unwrap().oResult;
                    
                    let oQuadrupleTemp = Quadruple::new(oOperator, 
                                                                    oResult, 
                                                                    None,
                                                                     None
                                                                );
                    self.oQuadruples.push(oQuadrupleTemp);
                    // Obtener el indice del quadruplo que ocupa saber el salto siguiente
                    let iQuadrupleIndex = self.oQuadruples.len().checked_sub(1); 
                    self.InsertJump(iQuadrupleIndex.unwrap());                                
                } ,
                Operator::Goto => {
                    let oOperator = self.oOperatorStack.pop().unwrap();      
                    let iEnd =self.oJumpsStack.pop().unwrap();
                    let iReturn =self.oJumpsStack.pop();               

                    
                    let oQuadrupleTemp = Quadruple::new(oOperator, 
                                                                    None, 
                                                                    None,
                                                                     iReturn
                                                                );
                    self.oQuadruples.push(oQuadrupleTemp);                    

                    // Obtener el indice del quadruplo que ocupa saber el salto siguiente
                    let iQuadrupleIndex = self.oQuadruples.len().checked_sub(1); 
                    self.InsertJump(iQuadrupleIndex.unwrap());    
                    self.FillGotoQuadruple(iEnd);
                }
   
                _ => { // Para cualquier otro caso
                    return false;
                }
            }
        }
        false 
    }

    pub fn FillGotoQuadruple(&mut self, iQuadrupleIndex: usize) -> bool{
        //Guardar en el Goto el indice a donde saltara
        self.oQuadruples[iQuadrupleIndex].oResult = Some(self.oQuadruples.len());
        return true;
    }

    pub fn FinishJump(&mut self) -> bool{
        let iEnd = self.oJumpsStack.pop().unwrap();
        return self.FillGotoQuadruple(iEnd);
    }

    pub fn print_table(&self) {

        println!("Saltos: {:?}", self.oJumpsStack);
        println!("Operandos: {:?}", self.oOperandStack);
        println!("Operadores: {:?}", self.oOperatorStack);

        println!("{:<5} | {:<12} | {:<10} | {:<10} | {:<10}", 
                 "Idx", "Operator", "Arg1", "Arg2", "Result");
        println!("{}", "-".repeat(60));

        for (i, q) in self.oQuadruples.iter().enumerate() {
            println!("{:<5} | {:<12?} | {:<10} | {:<10} | {:<10}", 
                i, 
                q.iOperator,
                match q.oArg1 {
                    Some(val) => val.to_string(),
                    None => "_".to_string(),
                },
                match q.oArg2 {
                    Some(val) => val.to_string(),
                    None => "_".to_string(),
                },
                match q.oResult {
                    Some(val) => val.to_string(),
                    None => "_".to_string(),
                }
            );
        }
    }


}
