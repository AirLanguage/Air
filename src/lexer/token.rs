use crate::lexer::lexer::{to_float, to_int, to_string};
use logos::Logos;

#[derive(Debug, Clone, Logos, PartialEq)]
pub enum Token {
  /* ------------------------ BRACKETS ------------------------ */
  #[token("(")]
  BracketLeft,
  #[token(")")]
  BracketRight,
  #[token("[")]
  BracketLeftSquare,
  #[token("]")]
  BracketRightSquare,
  #[token("{")]
  BracketLeftCurly,
  #[token("}")]
  BracketRightCurly,

  /* ------------------------ OPERATORS ------------------------ */
  #[token("+")]
  OperatorAddition,
  #[token("-")]
  OperatorSubtraction,
  #[token("*")]
  OperatorMultiplication,
  #[token("**")]
  OperatorExponentiation,
  #[token("/")]
  OperatorDivision,
  #[token("%")]
  OperatorModulus,
  #[token("++")]
  OperatorIncrement,
  #[token("--")]
  OperatorDecrement,

  #[token("=")]
  OperatorAssign,
  #[token("+=")]
  OperatorAssignAddition,
  #[token("-=")]
  OperatorAssignSubtraction,
  #[token("*=")]
  OperatorAssignMultiplication,
  #[token("/=")]
  OperatorAssignDivision,
  #[token(":=")]
  OperatorDeclareAssign,

  // Logic
  #[token("==")]
  OperatorEquals,
  #[token("?=")]
  OperatorAlmostEquals,
  #[token("!=")]
  OperatorNotEquals,
  #[token(">")]
  OperatorGreater,
  #[token("<")]
  OperatorLesser,
  #[token("<=")]
  OperatorGreaterOrEqual,
  #[token(">=")]
  OperatorLesserOrEqual,
  #[token("&&")]
  OperatorAnd,
  #[token("||")]
  OperatorOr,
  #[token("!")]
  OperatorNot,

  // Bit
  #[token("&")]
  OperatorBitAnd,
  #[token("|")]
  OperatorBitOr,
  #[token("~")]
  OperatorBitNot,
  #[token("^")]
  OperatorBitXor,
  #[token("<<<")]
  OperatorBitLeft,
  #[token(">>>")]
  OperatorBitRight,

  // Tenary
  // OperatorTenaryIf,
  // OperatorTenaryElse,

  // Pointer // ? Maybe
  // OperatorPointerAdress,   // &
  // OperatorPointerOperator, // *

  // Others operators
  // OperatorGlobal,        // $
  #[token("@")]
  OperatorDecorator,
  #[token("->")]
  OperatorArrow,
  #[token("=>")]
  OperatorFlatArrow,
  #[token("~>")]
  OperatorQuasiArrow,
  #[token(">>")]
  OperatorPipe,
  #[token("<<")]
  OperatorPipeReverse,
  #[token("..")]
  OperatorRange,
  #[token("...")]
  OperatorSpread,
  #[token(":")]
  OperatorInheritance,
  // OperatorUnderscore,
  // OperatorNullable,
  // OperatorNullForgiving,
  // OperatorTypeOf, // typeof || ?|>

  /* ------------------------ DATA TYPES ------------------------ */
  // Simple data types
  // TypeChar,
  // TypeString,
  // TypeComplex,
  // TypeBool,
  #[token("false")]
  TypeBoolFalse,
  #[token("true")]
  TypeBoolTrue,
  // TypeNull,
  // Complex data types
  // Array,

  /* ----------------------- CONDITIONS  ----------------------- */
  #[token("if")]
  ConditionsIf,
  #[token("else")]
  ConditionsElse,
  #[token("switch")]
  ConditionsSwitch,
  #[token("case")]
  ConditionsCase,

  /* ------------------------    LOOP    ------------------------ */
  #[token("for")]
  LoopFor, // for (let i = 0; i < n; i++) {}
  // LopForOf,  // for (let i of array) {}
  // LoopForIn, // for (let i in object) {}
  #[token("while")]
  LoopWhile, // while (i < n) {}
  #[token("loop")]
  LoopInf,
  #[token("break")]
  LoopBreak,
  #[token("continue")]
  LoopContinue,

  /* ------------------------  FUNCTIONS  ------------------------ */
  #[token("func")]
  FunctionFunc,

  /* ------------------------  Others  ------------------------ */
  #[regex(r"[a-zA-Z_]+", to_string)]
  Identifier(String),

  #[regex(r"[0-9]+", priority = 2, callback = to_int)]
  Int(i64),

  #[regex(r"([0-9]+[.])?[0-9]+", to_float)]
  Float(f64),

  #[error]
  #[regex(r"[ \t\n\f]+", logos::skip)]
  Error,
}
