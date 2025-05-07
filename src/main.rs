#![allow(warnings)]
extern crate lalrpop_util;
use lalrpop_util::lalrpop_mod;
mod parser;
mod ast;
mod directory;
lalrpop_mod!(#[allow(clippy::all)] pub grammar); // synthesized by LALRPOP

use parser::lexer::Lexer;
use crate::grammar::ProgramParser;

/*mod utils;
use utils::stack::Stack;
use utils::queue::Queue;
use std::collections::HashMap; //Libreria para el uso de Hashmap es mejor usarla que implementarlo de 0
*/

fn main() {
    let source_code = "    program prueba1;  
                            var a,b : int;
                            var c,d : float;
                            void func1(param1 : int, param2 : int)[
                                var a : float;
                                {
                                    print(a, b, ((c + d) + 5));
                                }
                            ];
                            main
                            {
                                if(a > (c * d) ) do
                                {
                                    func1(a, 1);                                
                                };
                                b = 1;
                            }
                            end";
    let lexer = Lexer::new(&source_code);

    let parser = ProgramParser::new();
    let ast = parser.parse(lexer);

    println!("{:?}", ast.unwrap());
}
