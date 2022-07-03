// use crate::context::*;
use crate::error::*;
use std::collections::HashMap;
// use crate::interpreter::*;
use crate::lexer::*;
use crate::node::*;
use crate::parser::*;
use crate::position::*;
use crate::token::*;
// use crate::value::*;
use std::fmt;

pub const DIGIT_STR: &'static str = "0123456789";

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
      // panic!();
      // println!("{:#?}", lex_res);
      // return lex_res.unwrap()
    }
  };
  println!("{:#?}", lex_res);

  // lex_res
  // std::process::exit(69);
  let mut parser = Parser::new(Parser {
    token_vec: lex_res.clone().unwrap(),
    arg_token_vec: arg_lex_res.clone().unwrap(),
    ..Default::default()
  });
  println!("arg lex res correct? {:#?}", arg_lex_res);
  println!("parser correct? {:#?}", parser);

  // let env_arg_vec = parse_args(env_arg_vec);
  // std::process::exit(69);

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

  Ok((arg_lex_res.unwrap().clone(), ast))

  // println!("ast correct? {:#?}", ast);

  /* let interpreter = Interpreter {};
  let context = Context {
    display_name: "<program>".to_string(),
    parent: None,
    parent_entry_pos: None,
  };
  let res = interpreter.visit(ast.node.clone().unwrap(), context);

  // (res.value, res.error)
  Ok(res) */

  /* Ok(Token::new(Token {
    // value: Some(3.0),
    ..Default::default()
  }))
  // println!("{text:#?} {filen:#?}")

  // println!("{:#?") */
}

/* pub fn parse_args(env_arg_vec: Vec<String>) -> Vec<Token> {
  for n in env_arg_vec {

  }
} */
