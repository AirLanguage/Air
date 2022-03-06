use crate::lexer::token::Token;
use logos::Lexer;

pub fn to_string(lex: &mut Lexer<Token>) -> Option<String> {
  Some(lex.slice().to_string())
}

pub fn to_int(lex: &mut Lexer<Token>) -> Option<i64> {
  let slice = lex.slice();
  let i: i64 = slice.parse().ok()?;
  Some(i)
}
pub fn to_float(lex: &mut Lexer<Token>) -> Option<f64> {
  let slice = lex.slice();
  let i: f64 = slice.parse().ok()?;
  Some(i)
}
