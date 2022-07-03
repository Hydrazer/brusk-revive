use crate::basic::*;
use crate::error::*;
use crate::position::*;
use crate::token::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Lexer {
  pub text: String,
  pub current_char: Option<char>,
  pub pos: Position,
  pub filen: String,
}

impl Default for Lexer {
  fn default() -> Self {
    Lexer {
      // text_list: "".to_string(),
      // current_text: "".to_string(),
      text: "".to_string(),
      current_char: None,
      filen: "".to_string(),
      pos: Position::new(Position {
        ..Default::default()
      }),
    }
  }
}

impl Lexer {
  pub fn new(lex: Lexer) -> Self {
    let mut l = lex;
    l.pos.filen = l.filen.clone();
    // l.pos.ftxt = l.filen.clone();
    l.advance();
    l
  }

  pub fn advance(&mut self) {
    self.pos.advance(self.current_char);
    self.current_char = self.text.chars().nth(self.pos.idx as usize);
  }

  pub fn make_tokens(&mut self) -> Result<Vec<Token>, Error> {
    let mut token_vec: Vec<Token> = vec![];

    while self.current_char.is_some() {
      let curr_char = self.current_char.unwrap();
      let curr_pos = Some(self.pos.clone());
      match curr_char {
        '_' => {
          token_vec.push(Token::new(Token {
            typ: TokenType::NEGATE,
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }

        's' => {
          token_vec.push(Token::new(Token {
            typ: TokenType::SHOW,
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }
        '?' => {
          token_vec.push(Token::new(Token {
            typ: TokenType::IF,
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }
        c if "₀₁₂₃₄".contains(c) => {
          token_vec.push(Token::new(Token {
            typ: TokenType::LINE_F,
            value: Some(TokenVal::LINE_F(c.to_string())),
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }
        c if "⁰".contains(c) => {
          token_vec.push(Token::new(Token {
            typ: TokenType::VAR,
            value: Some(TokenVal::VAR(c.to_string())),
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }
        'm' => {
          token_vec.push(Token::new(Token {
            typ: TokenType::MAP,
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }

        'ḣ' => {
          token_vec.push(Token::new(Token {
            typ: TokenType::HEADS,
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }
        '%' => {
          token_vec.push(Token::new(Token {
            typ: TokenType::MOD,
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }
        '\n' => {
          println!("pos ln {:#?}", self.pos.ln);
          token_vec.push(Token::new(Token {
            typ: TokenType::EOF,
            /* typ: match self.pos.ln {
              0 => TokenType::EOF,
              _ => TokenType::NEWLINE,
            }, */
            pos_start: curr_pos,
            ..Default::default()
          }));
          // token_vec.push(vec![]);
          self.advance();
          /* match self.current_char {
            Some(_) => token_vec.insert(0, vec![]),
            None => {}
          }; */
        }
        ' ' | '\t' => self.advance(),
        dig if DIGIT_STR.contains(dig) => token_vec.push(self.make_number()),
        '+' => {
          token_vec.push(Token::new(Token {
            typ: TokenType::PLUS,
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }
        '-' => {
          token_vec.push(Token::new(Token {
            typ: TokenType::MINUS,
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }
        '*' => {
          token_vec.push(Token::new(Token {
            typ: TokenType::MUL,
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }
        '/' => {
          token_vec.push(Token::new(Token {
            typ: TokenType::DIV,
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }
        '(' => {
          token_vec.push(Token::new(Token {
            typ: TokenType::LPAREN,
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }
        ')' => {
          token_vec.push(Token::new(Token {
            typ: TokenType::RPAREN,
            pos_start: curr_pos,
            ..Default::default()
          }));
          self.advance();
        }

        c => {
          let pos_start = self.pos.copy();
          self.advance();
          return Err(Error {
            pos_start: Some(pos_start),
            pos_end: Some(self.pos.copy()),
            err: ErrorType::IllegalChar(format!("'{c}'")),
          });
        }
      }
    }

    /* token_vec[0].push(Token::new(Token {
      typ: TokenType::NEWLINE,
      pos_start: curr_pos,
      ..Default::default()
    })); */

    // vec.retain(|&x| x % 2 == 0);
    // token_vec.retain(|&t| t.len() > 0);
    // token_vec.reverse();
    Ok(token_vec)
  }

  pub fn make_number(&mut self) -> Token {
    let mut dot_count = 0;
    let mut num_str = "".to_string();
    let pos_start = self.pos.copy();

    while self.current_char.is_some()
      && format!("{}.", DIGIT_STR).contains(self.current_char.unwrap())
    {
      let curr_char = self.current_char.unwrap();
      if curr_char == '.' {
        if dot_count == 1 {
          break;
        }

        dot_count += 1;
        num_str += ".";
      } else {
        num_str += curr_char.to_string().as_str();
      }
      self.advance();
    }

    match dot_count {
      0 => Token::new(Token {
        typ: TokenType::FLOAT,
        value: Some(TokenVal::FLOAT(num_str.parse::<f64>().unwrap())),
        pos_start: Some(pos_start.clone()),
        pos_end: Some(self.pos.clone()),
        ..Default::default()
      }),
      1 => Token::new(Token {
        typ: TokenType::FLOAT,
        value: Some(TokenVal::FLOAT(num_str.parse::<f64>().unwrap())),
        pos_start: Some(pos_start.clone()),
        pos_end: Some(self.pos.clone()),
        ..Default::default()
      }),
      _ => {
        unreachable!();
      }
    }
  }
}
