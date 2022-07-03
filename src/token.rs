use crate::error::*;
use crate::parser::*;
use crate::position::*;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Token {
  pub typ: TokenType,
  pub value: Option<TokenVal>,
  pub pos_start: Option<Position>,
  pub pos_end: Option<Position>,
}

impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.value.clone() {
      Some(v) => {
        write!(f, "{:?}:{:?}", self.typ, v)
      }

      None => {
        write!(f, "{:?}", self.typ)
      }
    }
  }
}

impl Default for Token {
  fn default() -> Self {
    Token {
      typ: TokenType::NAH,
      value: None,
      pos_start: None,
      pos_end: None,
    }
  }
}

impl Token {
  pub fn new(tok: Token) -> Self {
    // println!("createing new tok {:#?}", tok);
    let mut t = tok;
    if t.pos_start.is_some() && t.pos_end.is_none() {
      t.pos_end = t.pos_start.clone();

      let mut a = t.pos_end.clone().unwrap();
      a.advance(None);
      t.pos_end = Some(a);
    }
    t
  }

  pub fn get_val(&self, parser: Parser) -> Token {
    match self.value.clone() {
      // FLOAT(f64),
      Some(TokenVal::VAR(_str)) => parser.arg_token_vec["⁰¹²³⁴⁵⁶⁷⁸⁹"
        .chars()
        .position(|c| c.to_string() == _str)
        .unwrap()]
      .clone(),
      _ => unreachable!(), // LINE_F(String),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
  INT,
  FLOAT,
  DIV,
  MUL,
  PLUS,
  MINUS,
  RPAREN,
  LPAREN,
  EOF,
  NAH,
  NEWLINE,
  LINE_F,
  VAR,
  /* ZERO_F,
  ONE_F,
  TWO_F,
  THREE_F,
  FOUR_F, */
  MAP,
  IF,
  HEADS, // 1..=n
  MOD,
  SHOW,
  NEGATE,
}

impl TokenType {
  pub fn arg_vec(&self) -> Vec<TokenType> {
    match self.clone() {
      TokenType::NEGATE => {
        vec![TokenType::FLOAT]
      }
      TokenType::MUL | TokenType::DIV | TokenType::PLUS | TokenType::MINUS | TokenType::MOD => {
        vec![TokenType::FLOAT, TokenType::FLOAT]
      }

      TokenType::FLOAT => vec![],

      _ => unreachable!(),
    }
  }

  pub fn return_typ(&self, parser: Parser) -> Result<TokenType, Error> {
    println!("checking ret typ for {:#?}", self);
    match self.clone() {
      /* TokenType::TWO_F => match parser.clone().token_vec.into_iter().nth(2) {
        Some(tl) => tl[0].typ.return_typ(parser),

        None => Err(Error {
          err: ErrorType::IllegalSyntax("line don't exist".to_string()),
          pos_start: parser.clone().current_token.unwrap().pos_start,
          pos_end: parser.clone().current_token.unwrap().pos_end,
        }),
      }, */
      TokenType::FLOAT
      | TokenType::MUL
      | TokenType::DIV
      | TokenType::PLUS
      | TokenType::MINUS
      | TokenType::MOD
      | TokenType::NEGATE => {
        Ok(TokenType::FLOAT)
        // vec![TokenType::FLOAT, TokenType::FLOAT]
      }

      /* TokenType::TWO_F(tt) => {
        match tt {
          Some(t) => t
          // None => parser.token_vec[2][0].typ
        }
        // parser.token_vec[2][0].typ
      } */
      _ => unreachable!(),
    }
  }

  pub fn rust_typ(&self) -> String {
    match self {
      TokenType::FLOAT => "f64".to_string(),
      _ => unreachable!(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenVal {
  FLOAT(f64),
  VAR(String),
  LINE_F(String),
}
