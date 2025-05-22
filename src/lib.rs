//Archivo que exporta para que la carpeta tests pueda hacer pruebas correspondientes
#![allow(warnings)]
#[macro_use]
extern crate lalrpop_util;

use lalrpop_util::lalrpop_mod;

pub mod parser;
pub mod ast;
pub mod memory;
pub mod semantic;
pub mod utils;


lalrpop_mod!(#[allow(clippy::all)] pub grammar);

// Re-exporta el lexer y el parser
pub use parser::lexer::Lexer;
pub use grammar::ProgramParser;
pub use memory::directory::FunctionDirectory;
pub use memory::variables::VariableValueTable;
pub use ast::{DataType, Value};
pub use utils::stack::Stack;
pub use semantic::quadruples::QuadrupleList;
