use std::fmt;  // to implement the Display trait later
use std::num::{ParseFloatError, ParseIntError};
use logos::Logos;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    InvalidInteger(ParseIntError),
    InvalidFloat(ParseFloatError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}

impl From<ParseFloatError> for LexicalError {
  fn from(err: ParseFloatError) -> Self {
      LexicalError::InvalidFloat(err)
  }
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+", skip r"#.*\n?", error = LexicalError)]
pub enum Token {
  #[token("program")]
  KeywordProgram,
  #[token("main")]
  KeywordMain,
  #[token("end")]
  KeywordEnd,
  #[token("var")]
  KeywordVar,
  #[token("print")]
  KeywordPrint,
  #[token("void")]
  KeywordVoid,
  #[token("while")]
  KeywordWhile,
  #[token("if")]
  KeywordIf,
  #[token("else")]
  KeywordElse,
  #[token("do")]
  KeywordDo,
  #[token("int")]
  KeywordInt,
  #[token("float")]
  KeywordFloat,

  #[token("cte_int")]
  KeywordCTEInt,
  #[token("cte_float")]
  KeywordCTEFloat,

  #[regex("[\"][_0-9a-zA-Z]*[\"]")]
  CteString,

  #[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice().to_string())]
  Identifier(String),
  #[regex("[0-9]+", |lex| lex.slice().parse())]
  Integer(i64),
  #[regex("[0-9]+[.][1-9][0-9]*", |lex| lex.slice().parse())]
  Float(f64),
  #[token("(")]
  LParen,
  #[token(")")]
  RParen,
  #[token("[")]
  LSQRBracket,
  #[token("]")]
  RSQRBracket,
  #[token("{")]
  LBracket,
  #[token("}")]
  RBracket,
  #[token("=")]
  Assign,
  #[token(";")]
  Semicolon,
  #[token(":")]
  Colon,
  #[token(",")]
  Comma,

  #[token("\"")]
  Quote,

  #[token("+")]
  OperatorAdd,
  #[token("-")]
  OperatorSub,
  #[token("*")]
  OperatorMul,
  #[token("/")]
  OperatorDiv,

  #[token(">")]
  BiggerThan,
  #[token("<")]
  LessThan,
  #[token("!=")]
  NotEqual,

}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
    }
  }
  