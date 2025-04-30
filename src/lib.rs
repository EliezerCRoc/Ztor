//Archivo que exporta para que la carpeta tests pueda hacer pruebas correspondientes

#[macro_use]
extern crate lalrpop_util;

use lalrpop_util::lalrpop_mod;

pub mod parser;
pub mod ast;

lalrpop_mod!(#[allow(clippy::all)] pub grammar);

// Re-exporta el lexer y el parser
pub use parser::lexer::Lexer;
pub use grammar::ProgramParser;
