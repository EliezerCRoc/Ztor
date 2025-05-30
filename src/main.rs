#![allow(warnings)]
extern crate lalrpop_util;
use lalrpop_util::lalrpop_mod;
mod parser;
mod ast;
mod memory;
mod semantic;
mod utils;
mod compiler;

lalrpop_mod!(#[allow(clippy::all, warnings)] pub grammar); // synthesized by LALRPOP


use memory::directory::{FunctionDirectory, FunctionInfo};
use memory::variables::{VariableValueDirectory,VariableValueTable};
use parser::lexer::Lexer;
use crate::semantic::quadruples::QuadrupleList;
use crate::grammar::ScriptParser;
use crate::ast::{DataType, Operator, Value, Expression};
use crate::compiler::executer::Executer;


/*mod utils;
use utils::stack::Stack;
use utils::queue::Queue;
use std::collections::HashMap; //Libreria para el uso de Hashmap es mejor usarla que implementarlo de 0
*/

fn main() {
    //let mut oVariableValueTable = VariableValueTable::new();
    let mut oQuadrupleList = QuadrupleList::new();
    let mut oVariableValueDirectory = VariableValueDirectory::new();
    let mut oFuncDirectory = FunctionDirectory::new();





    let source_code = "    program prueba1;  
                            var a,b : int;
                            var c,d : float;
                            void a1(param1: int)[
                            var j: int;
                            {
                                j = 1000;
                                print(j);
                                a = 5;
                                print(a);
                                a2();
                            }];
                            void a2(param1: int)[
                            var j: int;
                            {
                                print(j);
                                a = 100;
                                print(a);
                            }];
                            main
                            {

                                a = 10;
                                while((a*5) > (10/2)) do {
                                    a = a - 1;
                                    print(a);
                                    
                                };
                                c = (2*5)/10;
                                print(c);
                                a1(a);

                            }
                            end";

    let lexer = Lexer::new(&source_code);

    let parser = ScriptParser::new();
    parser.parse(& mut oFuncDirectory, 
                &mut oVariableValueDirectory, 
                & mut oQuadrupleList, lexer);

    println!("{:?}", oFuncDirectory);
    println!("{:?}", oVariableValueDirectory);

    oQuadrupleList.print_table();
    //println!("{:?}", oVariableValueTable.get(oFuncDirectory.oFunctions["main"].oVariableDirectory["b"]));
    let mut oExecuter =Executer::new(&mut oFuncDirectory, 
                                       &mut oVariableValueDirectory, 
                                       &mut oQuadrupleList);
    
    oExecuter.executeQuadruple();

}
