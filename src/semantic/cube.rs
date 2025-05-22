#![allow(warnings)]
use std::collections::HashMap;
use crate::ast::{DataType, Operator, Value, Expression};



pub struct SemanticCube {
    oRules: HashMap<(DataType, DataType, Operator), Option<DataType>>,
}

impl SemanticCube {
    pub fn new() -> Self {
        let mut oRules = HashMap::new();

        
        oRules.insert((DataType::Int, DataType::Int, Operator::Add), Some(DataType::Int));
        oRules.insert((DataType::Int, DataType::Float, Operator::Add), Some(DataType::Float));
        oRules.insert((DataType::Float, DataType::Int, Operator::Add), Some(DataType::Float));
        oRules.insert((DataType::Float, DataType::Float, Operator::Add), Some(DataType::Float));

        oRules.insert((DataType::Int, DataType::Int, Operator::Sub), Some(DataType::Int));
        oRules.insert((DataType::Int, DataType::Float, Operator::Sub), Some(DataType::Float));
        oRules.insert((DataType::Float, DataType::Int, Operator::Sub), Some(DataType::Float));
        oRules.insert((DataType::Float, DataType::Float, Operator::Sub), Some(DataType::Float));

        oRules.insert((DataType::Int, DataType::Int, Operator::Mul), Some(DataType::Int));
        oRules.insert((DataType::Int, DataType::Float, Operator::Mul), Some(DataType::Float));
        oRules.insert((DataType::Float, DataType::Int, Operator::Mul), Some(DataType::Float));
        oRules.insert((DataType::Float, DataType::Float, Operator::Mul), Some(DataType::Float));

        oRules.insert((DataType::Int, DataType::Int, Operator::Div), Some(DataType::Int));
        oRules.insert((DataType::Int, DataType::Float, Operator::Div), Some(DataType::Float));
        oRules.insert((DataType::Float, DataType::Int, Operator::Div), Some(DataType::Float));
        oRules.insert((DataType::Float, DataType::Float, Operator::Div), Some(DataType::Float));

        oRules.insert((DataType::Int, DataType::Int, Operator::GreaterThan), Some(DataType::Bool));
        oRules.insert((DataType::Int, DataType::Float, Operator::GreaterThan), Some(DataType::Bool));
        oRules.insert((DataType::Float, DataType::Int, Operator::GreaterThan), Some(DataType::Bool));
        oRules.insert((DataType::Float, DataType::Float, Operator::GreaterThan), Some(DataType::Bool));

        oRules.insert((DataType::Int, DataType::Int, Operator::LessThan), Some(DataType::Bool));
        oRules.insert((DataType::Int, DataType::Float, Operator::LessThan), Some(DataType::Bool));
        oRules.insert((DataType::Float, DataType::Int, Operator::LessThan), Some(DataType::Bool));
        oRules.insert((DataType::Float, DataType::Float, Operator::LessThan), Some(DataType::Bool));

        oRules.insert((DataType::Int, DataType::Int, Operator::NotEqual), Some(DataType::Bool));
        oRules.insert((DataType::Int, DataType::Float, Operator::NotEqual), Some(DataType::Bool));
        oRules.insert((DataType::Float, DataType::Int, Operator::NotEqual), Some(DataType::Bool));
        oRules.insert((DataType::Float, DataType::Float, Operator::NotEqual), Some(DataType::Bool));
        
        

        SemanticCube { oRules }
    }

    pub fn result_type(&self, left: DataType, right: DataType, op: Operator) -> Option<DataType> {
        self.oRules.get(&(left, right, op)).cloned().unwrap_or(None)
    }
}
