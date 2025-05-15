#![allow(warnings)]
extern crate lalrpop_util;
use lalrpop_util::lalrpop_mod;
mod parser;
mod ast;
mod memory;
mod semantic;
mod utils;
lalrpop_mod!(#[allow(clippy::all)] pub grammar); // synthesized by LALRPOP

use memory::directory::{FunctionDirectory, FunctionInfo};
use memory::variables::VariableValueTable;
use parser::lexer::Lexer;
use crate::semantic::quadruples::QuadrupleList;
use crate::grammar::ScriptParser;
use crate::ast::{DataType, Operator, Value, Expression};


/*mod utils;
use utils::stack::Stack;
use utils::queue::Queue;
use std::collections::HashMap; //Libreria para el uso de Hashmap es mejor usarla que implementarlo de 0
*/

fn main() {
    let mut oFuncDirectory = FunctionDirectory::new();
    let mut oVariableValueTable = VariableValueTable::new();
    let mut oQuadrupleList = QuadrupleList::new();



    let source_code = "    program prueba1;  
                            var a,b : int;
                            var c,d : float;
                            main
                            {
                                
                                b = a + 1 * 5;
                            }
                            end";

    let lexer = Lexer::new(&source_code);

    let parser = ScriptParser::new();
    parser.parse(& mut oFuncDirectory, & mut oVariableValueTable, & mut oQuadrupleList, lexer);

    println!("{:?}", oFuncDirectory);
    //println!("{:?}", oVariableValueTable.get(oFuncDirectory.oFunctions["main"].oVariableDirectory["b"]));
    println!("{:?}", oQuadrupleList.oQuadruples);


}
