#[macro_use]
extern crate lalrpop_util;
use lalrpop_util::lalrpop_mod;


mod parser;
mod ast;
lalrpop_mod!(#[allow(clippy::all)] pub grammar); // synthesized by LALRPOP


use parser::lexer::Lexer;
use crate::grammar::ProgramParser;

/*mod utils;
use utils::stack::Stack;
use utils::queue::Queue;
use std::collections::HashMap; //Libreria para el uso de Hashmap es mejor usarla que implementarlo de 0
*/



fn main() {
    let source_code = "    program a123;  
                            var a,b : int;
                            var c,d : float;
                            void func1(a : int, b : int)[
                                var c,d : float;
                                {
                                    print(a, b, ((c + d) + 5));
                                }
                            ];
                            main
                            {
                                if(a > (c + d)) do
                                {
                                    func1(a, b);                                
                                };
                                b = 1;
                            }
                            end";
    let lexer = Lexer::new(&source_code);

    let parser = ProgramParser::new();
    let ast = parser.parse(lexer);

    println!("{:?}", ast.unwrap());
}
