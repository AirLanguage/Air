use crate::lexer::token::Token;
use logos::Logos;

#[test]
fn it_can_recognize_reserved_keywords() {
  let mut lexer = Token::lexer(
    "if else switch case for while loop in not in break continue func return false true null",
  );

  assert_eq!(lexer.next(), Some(Token::FunctionFunc))
}

#[test]
fn it_can_recognize_operators() {
  let mut lexer = Token::lexer(", : . ( ) [ ] { } + - * / ** % ++ -- = += -= *= /= := == ?= != > < >= <= && || ! & | ~ ^ <<< >>> @ -> => ~> >> << .. ... _");

  assert_eq!(lexer.next(), Some(Token::Comma));
  assert_eq!(lexer.next(), Some(Token::Colon));
  assert_eq!(lexer.next(), Some(Token::Dot));
  assert_eq!(lexer.next(), Some(Token::BracketLeft));
  assert_eq!(lexer.next(), Some(Token::BracketRight));
  assert_eq!(lexer.next(), Some(Token::BracketLeftSquare));
  assert_eq!(lexer.next(), Some(Token::BracketRightSquare));
  assert_eq!(lexer.next(), Some(Token::BracketLeftCurly));
  assert_eq!(lexer.next(), Some(Token::BracketRightCurly));

  assert_eq!(lexer.next(), Some(Token::OperatorPlus));
  assert_eq!(lexer.next(), Some(Token::OperatorMinus));
  assert_eq!(lexer.next(), Some(Token::OperatorMulti));
  assert_eq!(lexer.next(), Some(Token::OperatorSlash));
  assert_eq!(lexer.next(), Some(Token::OperatorPower));
  assert_eq!(lexer.next(), Some(Token::OperatorPercent));
  assert_eq!(lexer.next(), Some(Token::OperatorIncrement));
  assert_eq!(lexer.next(), Some(Token::OperatorDecrement));

  assert_eq!(lexer.next(), Some(Token::OperatorAssign));
  assert_eq!(lexer.next(), Some(Token::OperatorAssignAddition));
  assert_eq!(lexer.next(), Some(Token::OperatorAssignSubtraction));
  assert_eq!(lexer.next(), Some(Token::OperatorAssignMultiplication));
  assert_eq!(lexer.next(), Some(Token::OperatorAssignDivision));
  assert_eq!(lexer.next(), Some(Token::OperatorDeclareAssign));

  assert_eq!(lexer.next(), Some(Token::OperatorEquals));
  assert_eq!(lexer.next(), Some(Token::OperatorAlmostEquals));
  assert_eq!(lexer.next(), Some(Token::OperatorNotEquals));
  assert_eq!(lexer.next(), Some(Token::OperatorGreater));
  assert_eq!(lexer.next(), Some(Token::OperatorLesser));
  assert_eq!(lexer.next(), Some(Token::OperatorGreaterOrEqual));
  assert_eq!(lexer.next(), Some(Token::OperatorLesserOrEqual));
  assert_eq!(lexer.next(), Some(Token::OperatorAnd));
  assert_eq!(lexer.next(), Some(Token::OperatorOr));
  assert_eq!(lexer.next(), Some(Token::OperatorNot));

  assert_eq!(lexer.next(), Some(Token::OperatorBitAnd));
  assert_eq!(lexer.next(), Some(Token::OperatorBitOr));
  assert_eq!(lexer.next(), Some(Token::OperatorBitNot));
  assert_eq!(lexer.next(), Some(Token::OperatorBitXor));
  assert_eq!(lexer.next(), Some(Token::OperatorBitLeft));
  assert_eq!(lexer.next(), Some(Token::OperatorBitRight));

  assert_eq!(lexer.next(), Some(Token::OperatorDecorator));
  assert_eq!(lexer.next(), Some(Token::OperatorArrow));
  assert_eq!(lexer.next(), Some(Token::OperatorFlatArrow));
  assert_eq!(lexer.next(), Some(Token::OperatorQuasiArrow));
  assert_eq!(lexer.next(), Some(Token::OperatorPipe));
  assert_eq!(lexer.next(), Some(Token::OperatorPipeReverse));
  assert_eq!(lexer.next(), Some(Token::OperatorRange));
  assert_eq!(lexer.next(), Some(Token::OperatorSpread));
  assert_eq!(lexer.next(), Some(Token::OperatorUnderscore));
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

#[test]
fn it_can_recognise_strings() {
  let mut lexer = Token::lexer(r##""TypeScript" "Go" "C#" "C++""##);
  assert_eq!(
    lexer.next(),
    Some(Token::String(r##"TypeScript"##.to_owned()))
  );
  assert_eq!(lexer.next(), Some(Token::String(r##"Go"##.to_owned())));
  assert_eq!(lexer.next(), Some(Token::String(r##"C#"##.to_owned())));
  assert_eq!(lexer.next(), Some(Token::String(r##"C++"##.to_owned())));
}