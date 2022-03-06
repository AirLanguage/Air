use crate::lexer::token::Token;
use logos::Logos;

#[test]
fn it_can_recognize_reserved_keywords() {
  let mut lexer = Token::lexer("func");

  assert_eq!(lexer.next(), Some(Token::FunctionFunc))
}

#[test]
fn it_can_recognise_identifiers() {
  let mut lexer = Token::lexer("hello_world HelloWorld helloworld helloworLD");
  assert_eq!(
    lexer.next(),
    Some(Token::Identifier("hello_world".to_owned()))
  );
  assert_eq!(
    lexer.next(),
    Some(Token::Identifier("HelloWorld".to_owned()))
  );
  assert_eq!(
    lexer.next(),
    Some(Token::Identifier("helloworld".to_owned()))
  );
  assert_eq!(
    lexer.next(),
    Some(Token::Identifier("helloworLD".to_owned()))
  );
}

#[test]
fn it_can_recognise_numbers() {
  let mut lexer = Token::lexer("2137 420 69.69 2.718281828459045");
  assert_eq!(lexer.next(), Some(Token::Int(2137)));
  assert_eq!(lexer.next(), Some(Token::Int(420)));
  assert_eq!(lexer.next(), Some(Token::Float(69.69)));
  assert_eq!(lexer.next(), Some(Token::Float(2.718281828459045)));
}
