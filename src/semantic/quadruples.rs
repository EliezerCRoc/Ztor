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
    pub oArg1: usize, 
    pub oArg2: usize,
    pub oResult: usize
}

impl Quadruple{
    pub fn new(_iOperator: Operator, _oArg1: usize, _oArg2: usize, _oResult: usize) -> Self {
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

    //Stack | Vector Polaco, puede contener operadors o usize (direcciones de memoria)
    pub oVecPolac: Vec<PolacItem>,

    //Vector de cuadruplos, aqui se registraran todas las acciones
    pub oQuadruples: Vec<Quadruple>,
}

impl QuadrupleList {

    pub fn new() -> Self {
        Self {
            oVariableTempTable: HashMap::new(),
            oOperatorStack: Stack::new(),
            oOperandStack: Stack::new(),    
            oVecPolac: Vec::new(),      
            oQuadruples: Vec::new(),
        }
    }

    pub fn InsertOperand(&mut self, _uVar: usize){
        self.oOperandStack.push(_uVar);
    }

    pub fn InsertOperator(&mut self, _oOperator: Operator){
        self.oOperatorStack.push(_oOperator);
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

     pub fn GenerateQuadruple(&mut self, memory: &mut VariableValueTable) {
        if let Some(operator_ref) = self.oOperatorStack.peek() {
            match operator_ref {
                Operator::Add | Operator::Sub |
                Operator::Mul | Operator::Div  => {
                    let right_operand = self.oOperandStack.pop().unwrap();
                    let operator = self.oOperatorStack.pop().unwrap();
                    let left_operand = self.oOperandStack.pop().unwrap();

                    let result_type = self.CheckSemanticCube(right_operand, left_operand, operator);
                    if let Some(r_type) = result_type {
                        // Usar insert con valor "placeholder"
                        let placeholder_value = match r_type {
                            DataType::Int => Value::Int(0),
                            DataType::Float => Value::Float(0.0),
                            DataType::Bool => Value::Bool(false),
                        };
                        let result_address = memory.insert(placeholder_value, r_type);

                        let quad = Quadruple::new(operator, left_operand, right_operand, result_address);
                        self.oQuadruples.push(quad);
                        self.oOperandStack.push(result_address);
                    } else {
                        panic!("Type mismatch: {:?} {:?} {:?}", left_operand, operator, right_operand);
                    }
                }
                _ => {}
            }
        }
    }


}
