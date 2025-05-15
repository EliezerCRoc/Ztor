use core::fmt;
use core::fmt::Error;
use std::collections::{HashMap, VecDeque};


#[derive(Clone, Debug, PartialEq)]
pub enum Header {
    Program {id: String}
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
  Variable { name: String, value: Box<Expression> },
  Print { value: Box<Expression> },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
  Integer(i64),
  Float(f64),  
  BinaryOperation {
    left: Box<Expression>,
    operator: Operator,
    right: Box<Expression>,
  },
  UnaryOperation {
    operator: Operator,
    expr: Box<Expression>,
  },
  Identifier(String),
  Constant(Value),
}


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Id(String),           
    None,             
}

impl Value {
    pub fn get_type(&self) -> DataType {
        match self {
            Value::Int(_) => DataType::Int,
            Value::Float(_) => DataType::Float,
            Value::Bool(_) => DataType::Bool,
            Value::Id(_) => {                
                panic!("get_type: no se puede determinar tipo de un identificador sin tabla de símbolos");
            }
            Value::None => {            
                panic!("Tipo None no tiene tipo definido")
            }
        }
    }
}

#[derive(Copy,Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataType {
    Int,
    Float,
    Bool,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /   
    GreaterThan, // >
    LessThan, // <
    NotEqual, // !=
}