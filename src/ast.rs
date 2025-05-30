#![allow(warnings)]
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

#[derive(Copy,Debug,PartialEq, Clone)]
#[repr(usize)]
pub enum Context{
    Global = 10000,
    Constant = 20000,
    Local = 30000,
    Temp = 40000
}

impl Context {
    pub fn getIndex( uIndex: usize) -> usize{  
    
        if uIndex >= Context::Constant as usize && uIndex < Context::Local as usize {
            return uIndex - (Context::Constant as usize);
        } else if uIndex >= Context::Local as usize && uIndex < Context::Temp as usize {
            return uIndex - (Context::Local as usize);
        } else if uIndex >= Context::Temp as usize {
            return uIndex - (Context::Temp as usize);
        } else if uIndex >= Context::Global as usize {
            return uIndex - (Context::Global as usize);
        }
        panic!("Error: Direccion Invalida")
    }
}

#[derive( Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),           
    None,             
}

impl From<i64> for Value {
    fn from(val: i64) -> Self {
        Value::Int(val)
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Float(val)
    }
}

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Value::Bool(val)
    }
}

impl From<String> for Value {
    fn from(val: String) -> Self {
        Value::String(val)
    }
}


impl Value {
    pub fn get_type(&self) -> DataType {
        match self {
            Value::Int(_) => DataType::Int,
            Value::Float(_) => DataType::Float,
            Value::Bool(_) => DataType::Bool,
            Value::String(_) => DataType::String,
            
            Value::None => {            
                panic!("Tipo None no tiene tipo definido")
            }
        }
    }
}

#[derive(Copy,Debug, Clone, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum DataType {
    Int = 0,
    Float = 4000,
    Bool = 8000,
    String = 9000
}
impl DataType {
    pub fn DefaultValue(&self) -> Value {
        match self {
            DataType::Int => Value::Int(0),
            DataType::Float => Value::Float(0.0),
            DataType::Bool => Value::Bool(false),
            DataType::String => Value::String("".to_string())
        }
    }
    pub fn GetTypeFromContext(uIndex: usize) -> DataType {
        let _uIndex = Context::getIndex(uIndex);
        return DataType::GetType(_uIndex)
    }

    pub fn GetType(uIndex: usize) -> DataType {
        let _uIndex = uIndex;
        if ((_uIndex >= (DataType::Float as usize)) && (_uIndex < (DataType::Bool as usize)) ){
            return DataType::Float;
        }
        else if(_uIndex >= (DataType::Bool as usize) && (_uIndex < (DataType::String as usize))){
            return DataType::Bool;
        }      
        else if(_uIndex >= (DataType::String as usize)){
            return  DataType::String;
        }  
        return DataType::Int;
    }
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
    Parenthesis,
    Goto,
    GotoF,
    GotoV,
    Assign,
    Print,
    Era,
    GoSub,
    Return,
    FinishFunction
}