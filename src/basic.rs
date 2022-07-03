use crate::error::*;
use std::collections::HashMap;
use crate::lexer::*;
use crate::node::*;
use crate::parser::*;
use crate::position::*;
use crate::token::*;
use std::fmt;

pub const DIGIT_STR: &'static str = "0123456789";
pub const VAR_STR: &'static str = "⁰¹²³⁴⁵⁷⁸⁹";

pub fn run(
  text: String,
  filen: String,
  env_arg_vec: Vec<String>,
) -> Result<(Vec<Token>, HashMap<usize, Ast>), Error> {
  let mut arg_lexer = Lexer::new(Lexer {
    filen: "<args>".to_string(),
    text: env_arg_vec.join(" "),
    ..Default::default()
  });

  let arg_lex_res = arg_lexer.make_tokens();

  match arg_lex_res.clone() {
    Ok(res) => {}

    Err(err) => {
      println!("arg lexer err: {:#?}", arg_lex_res);
      std::process::exit(69);
    }
  };

  let mut lexer = Lexer::new(Lexer {
    filen: filen,
    text: text,
    ..Default::default()
  });
  let lex_res = lexer.make_tokens();

  match lex_res.clone() {
    Ok(res) => {}

    Err(err) => {
      println!("lexer err: {:#?}", lex_res);
      std::process::exit(69);
    }
  };
  println!("{:#?}", lex_res);

  let mut parser = Parser::new(Parser {
    token_vec: lex_res.clone().unwrap(),
    arg_token_vec: arg_lex_res.clone().unwrap(),
    ..Default::default()
  });
  println!("arg lex res correct? {:#?}", arg_lex_res);
  println!("parser correct? {:#?}", parser);

  let ast = parser.ast();
  println!("ebic ast {:#?}", ast);
  match ast.get(&0) {
    Some(r) => {
      match r.error.clone() {
        Some(e) => return Err(e),
        None => {} // None => Ok(ast.node.unwrap())
      }
    }
    None => {}
  };

  Ok((arg_lex_res.clone().unwrap(),ast))
}

