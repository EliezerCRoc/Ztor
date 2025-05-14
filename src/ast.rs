use core::fmt;
use core::fmt::Error;
use std::collections::{HashMap, VecDeque};
use crate::semantic::datatype::{Operator, Value};


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
