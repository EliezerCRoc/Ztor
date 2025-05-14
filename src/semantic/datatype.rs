#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Id(String),       
    Temp(String),     
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
            Value::Temp(_) => {
                DataType::Int
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

