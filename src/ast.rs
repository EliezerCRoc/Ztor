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
  Variable(String),
  BinaryOperation {
    lhs: Box<Expression>,
    operator: Operator,
    rhs: Box<Expression>,
  },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
}
