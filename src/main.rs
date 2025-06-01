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
                            void Factorial(n: int)[
                            var iFactorialNum,i: int;                            
                            {
                                i = 1;
                                iFactorialNum = 1;
                                while(i < (n+1)) do {
                                    iFactorialNum = i * iFactorialNum;
                                    i = i + 1;
                                };                                
                                print(iFactorialNum);
                                
                            }];

                            void Fibbonacci(i:int, n: int)[
                            var prev1,prev2,newFibo: int;                            
                            {
                                prev1 = 1;                            
                                print(i);
                                while(i < (n+1)) do {
                                    newFibo = prev1 + prev2;
                                    print(newFibo);
                                    prev2 = prev1;
                                    prev1 = newFibo;
                                    i = i+1;
                                };                                                                
                            }];

                            main
                            {
                                Factorial(10);
                                Fibbonacci(1, 15);

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
