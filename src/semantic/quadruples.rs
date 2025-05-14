use crate::semantic::datatype::{DataType, Operator, Value};
use crate::semantic::cube::SemanticCube;
use crate::ast::Expression;

use crate::utils::stack::Stack;

pub struct Quadruple{
    pub iOperator: Operator,
    pub oArg1: Value, 
    pub oArg2: Value,
    pub oResult: Value
}

impl Quadruple{
    pub fn new(_iOperator: Operator, _oArg1: Value, _oArg2: Value) -> Self {
        let mut _oResult: Value = Value::Int(1);

        // match _iOperator{
        //     Operator::Add => {_oResult = _oArg1 + _oArg1},
        //     Operator::Sub => {_oResult = _oArg1 - _oArg1}
        //     Operator::Mul => {_oResult = _oArg1 * _oArg1}
        //     Operator::Div => {_oResult = _oArg1 / _oArg1}

        // }

        Self { 
            iOperator: _iOperator, 
            oArg1: _oArg1, 
            oArg2: _oArg2, 
            oResult: _oResult
        }
    } 
}

pub struct QuadrupleList {
    pub oOperatorStack: Stack<Operator>,
    pub oOperandStack: Stack<Value>,
    pub oTypeStack: Stack<DataType>,
    pub oQuadruples: Vec<Quadruple>,
}

impl QuadrupleList {

    pub fn new() -> Self {
        Self {
            oOperatorStack: Stack::new(),
            oOperandStack: Stack::new(),
            oTypeStack: Stack::new(),
            oQuadruples: Vec::new(),
        }
    }

    pub fn generate_from_expr(&mut self, expr: Expression) -> Value {
        match expr {
            Expression::Integer(n) => {
                let value = Value::Int(n);
                self.oOperandStack.push(value.clone());
                self.oTypeStack.push(DataType::Int);
                value
            }
            Expression::Float(f) => {
                let value = Value::Float(f);
                self.oOperandStack.push(value.clone());
                self.oTypeStack.push(DataType::Float);
                value
            }
            Expression::Identifier(name) => {
                let value = Value::Id(name.clone());
                // Aquí deberías buscar el tipo real de la variable en tu tabla de símbolos
                let dtype = DataType::Int; // Placeholder
                self.oOperandStack.push(value.clone());
                self.oTypeStack.push(dtype);
                value
            }
            Expression::Constant(val) => {
                let dtype = val.get_type(); // Implementa esto si aún no lo tienes
                self.oOperandStack.push(val.clone());
                self.oTypeStack.push(dtype);
                val
            }
            Expression::UnaryOperation { operator, expr } => {
                let operand_val = self.generate_from_expr(*expr);
                let operand_type = self.oTypeStack.pop().unwrap();

                let mut cube = SemanticCube::new();
                let result_type = cube
                    .result_type(operand_type.clone(), operand_type.clone(), operator)
                    .expect("Operación unaria inválida");

                let result = Value::Temp(format!("t{}", self.oQuadruples.len()));
                let quad = Quadruple {
                    iOperator: operator,
                    oArg1: operand_val.clone(),
                    oArg2: Value::None,
                    oResult: result.clone(),
                };

                self.oOperandStack.push(result.clone());
                self.oTypeStack.push(result_type);
                self.oQuadruples.push(quad);

                result
            }
            Expression::BinaryOperation { left, operator, right } => {
                let left_val = self.generate_from_expr(*left);
                let right_val = self.generate_from_expr(*right);

                let right_type = self.oTypeStack.pop().unwrap();
                let left_type = self.oTypeStack.pop().unwrap();

                let mut cube = SemanticCube::new();
                let result_type = cube
                    .result_type(left_type, right_type, operator)
                    .expect("Tipos incompatibles en operación binaria");

                let result = Value::Temp(format!("t{}", self.oQuadruples.len()));
                let quad = Quadruple {
                    iOperator: operator,
                    oArg1: left_val.clone(),
                    oArg2: right_val.clone(),
                    oResult: result.clone(),
                };

                self.oOperandStack.push(result.clone());
                self.oTypeStack.push(result_type);
                self.oQuadruples.push(quad);

                result
            }
        }
    }
}
