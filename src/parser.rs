use crate::error::*;
use crate::node::*;
use crate::position::*;
use crate::token::*;
use std::collections::HashMap;

use lazy_static::lazy_static;
use std::sync::Mutex;
use std::sync::MutexGuard;

lazy_static! {
  static ref FN_HASH: Mutex<HashMap<usize, Ast>> = Mutex::new(HashMap::new());
  static ref PARSE_LINE_ARG_HASH: Mutex<HashMap<usize, Vec<TokenType>>> =
    Mutex::new(HashMap::new());
}

#[derive(Debug, Clone)]
pub struct Parser {
  pub token_vec: Vec<Token>,
  pub pos: Position,
  pub curr_tok: Option<Token>,
  pub arg_token_vec: Vec<Token>,
}

impl Default for Parser {
  fn default() -> Self {
    Parser {
      pos: Position::new(Position {
        ..Default::default()
      }),
      token_vec: vec![],
      arg_token_vec: vec![],
      curr_tok: None,
    }
  }
}

impl Parser {
  pub fn new(parser: Self) -> Self {
    let mut p = parser;
    p.advance();
    p
  }

  pub fn advance(&mut self) -> Option<Token> {
    self.pos.advance(None);

    self.curr_tok = self
      .token_vec
      .clone()
      .into_iter()
      .nth(self.pos.col as usize);

    self.curr_tok.clone()
  }

  pub fn ast(&mut self) -> HashMap<usize, Ast> {
    while self.curr_tok.is_some() && self.curr_tok.clone().unwrap().typ != TokenType::EOF {
      let mut res = self.expr();
      FN_HASH
        .lock()
        .unwrap()
        .insert(self.pos.ln as usize, res.clone());

      match self.curr_tok.clone() {
        Some(ct) => {
          match ct.clone().typ {
            TokenType::EOF => {}
            // TokenType::NEWLINE => {self.pos.col = -1; self.pos.ln += 1;}
            _ => {
              FN_HASH.lock().unwrap().insert(
                0,
                res.failure(Error {
                  pos_start: self.curr_tok.clone().unwrap().pos_start,
                  pos_end: self.curr_tok.clone().unwrap().pos_end,
                  err: ErrorType::IllegalSyntax("Expected '+' '-' '*' '/'".to_string()),
                }),
              );
              return FN_HASH.lock().unwrap().clone();
            }
          };
        }

        None => unreachable!(),
      }
    }

    FN_HASH.lock().unwrap().clone()
  }

  pub fn expr(&mut self) -> Ast {
    self.binop()
  }

  fn factor(&mut self) -> Ast {
    let mut res = Ast {
      ..Default::default()
    };
    let tok = self.curr_tok.clone();

    match tok.clone().unwrap().typ {
      TokenType::INT | TokenType::FLOAT => {
        // need to put a res.register() around this later but weird cuz not same type
        self.advance();

        res.success(
          Some(Node::NumberNode {
            tok: tok.clone().unwrap().clone(),
            pos_start: tok.clone().unwrap().pos_start,
            pos_end: tok.clone().unwrap().pos_end,
          }),
          vec![],
          None,
        )
      }

      _ => res.failure(Error {
        pos_start: tok.clone().unwrap().pos_start,
        pos_end: tok.clone().unwrap().pos_end,
        err: ErrorType::IllegalSyntax("Expected Int or Float".to_string()),
      }),
    }
  }

  fn binop(&mut self) -> Ast {
    println!("new binop");
    let mut res = Ast {
      ..Default::default()
    };

    let (ps, pe) = match self.curr_tok.clone() {
      Some(t) => (t.pos_start, t.pos_end),
      None => (None, None),
    };

    while self.curr_tok.clone().is_some() {
      let curr_tok = self.curr_tok.clone().unwrap();
      match curr_tok.clone().typ {
        TokenType::VAR => {
          self.advance();
          // return res.success(None, vec![], None);
          println!("hit ze curr tok {:#?}", curr_tok);
          return res.success(
            Some(Node::NumberNode {
              tok: curr_tok.clone(),
              // value:
              pos_start: curr_tok.clone().pos_start,
              pos_end: curr_tok.clone().pos_end,
            }),
            vec![],
            None,
          );
        }
        /* TokenType::NEGATE => {

        } */
        TokenType::PLUS
        | TokenType::MINUS
        | TokenType::DIV
        | TokenType::MUL
        | TokenType::MOD
        | TokenType::NEGATE => {
          self.advance();

          let mut arg_vec = vec![];

          for (idx_c, arg_typ) in curr_tok.clone().typ.arg_vec().into_iter().enumerate() {
            let (res_c, child) = res.register(self.binop());
            // println!("huh whatdis chidl {:#?}", child);

            arg_vec.push(child.clone());
            if child.is_none() {
              println!("child is none check curtok {:#?}", self.curr_tok);
              return res.failure(Error {
                pos_start: ps,
                pos_end: match self.curr_tok.clone() {
                  Some(ct) => ct.pos_start,
                  None => None,
                },
                err: ErrorType::IllegalSyntax("expected argument".to_string()),
                // err: ErrorType::IllegalSyntax("Passed in Wrong Type".to_string()),
              });
              /* //
              let last_idx = arg_vec.clone().len() - 1;

              for i in curr_tok.clone().typ.arg_vec().into_iter().skip(idx_c + 1) {
                arg_vec.push(None);
              } */
              // println!("deez nuts line fn arg vec {:#?}", self.line_fn_arg_vec);
              /* self.line_fn_arg_vec[self.tok_idx as usize] = match self.line_fn_arg_vec[self.tok_idx as usize].clone() {
                None => Some(curr_tok.clone().typ.arg_vec().into_iter().skip(idx_c).map(|tt| tt).collect::<Vec<_>>()),
                Some(v) => {
                  let mut l = v;

                  l.append(&mut curr_tok.clone().typ.arg_vec().into_iter().skip(idx_c).map(|tt| tt).collect::<Vec<_>>());
                  Some(l)
               }
              }; */
              break;
            } else {
              let typer = match child.clone().unwrap() {
                Node::BinOpNode { op_tok, .. } => op_tok,
                Node::NumberNode { tok, .. } => tok,
              };
              match typer.typ {
                _ => {}
              }
            }

            if res.error.is_some() {
              return res;
            }

            let child_ret_typ = match child.clone().unwrap() {
              Node::BinOpNode { op_tok, .. } => op_tok.typ.return_typ(self.clone()).unwrap(),

              Node::NumberNode { tok, .. } => {
                TokenType::FLOAT
                // tok.typ.return_typ(self.clone()).unwrap()
              }
            };

            if child_ret_typ != arg_typ {
              // return res.error
              return res.failure(Error {
                pos_start: ps,
                pos_end: match self.curr_tok.clone() {
                  Some(ct) => ct.pos_start,
                  None => None,
                },
                err: ErrorType::IllegalSyntax("Passed in Wrong Type".to_string()),
              });
            }
          }
          // println!("arg vec clon; {:#?}", arg_vec.clone());

          let parent = Some(Node::BinOpNode {
            op_tok: curr_tok.clone(),
            arg_vec: arg_vec.clone(),
            // pos_start: None,
            pos_start: match arg_vec.clone().into_iter().nth(0) {
              Some(l) => match l.clone() {
                Some(Node::BinOpNode { pos_start, .. }) => pos_start,

                Some(Node::NumberNode { pos_start, .. }) => pos_start,
                _ => None,
              },
              None => None,
            },
            pos_end: match arg_vec.clone().into_iter().last() {
              Some(r) => match r.clone() {
                Some(Node::BinOpNode { pos_end, .. }) => pos_end,

                Some(Node::NumberNode { pos_end, .. }) => pos_end,
                None => None,
              },
              None => None,
            },
          });

          return res.success(parent, vec![], None);
        }

        TokenType::INT | TokenType::FLOAT => {
          let fac = self.factor();
          println!("reach int / float {:#?}", fac);
          return fac;
        }
        TokenType::NEWLINE => {
          break;
          // self.advance();
          // break;
        }
        TokenType::EOF => {
          break;
        }
        _ => unreachable!(),
      }
    }

    res.success(None, vec![], None)
  }
}

#[derive(Clone, Debug)]
pub struct Ast {
  pub error: Option<Error>,
  pub node: Option<Node>,
  /* pub line_fn_arg_vec: Vec<Option<Vec<TokenType>>>,
  pub parse_line_fn_hash: HashMap<usize, ParseResult>, */
}

impl Default for Ast {
  fn default() -> Ast {
    Ast {
      error: None,
      node: None,
      /* line_fn_arg_vec: vec![],
      parse_line_fn_hash: HashMap::new() */
    }
  }
}

impl Ast {
  pub fn new(parse_res: Self) -> Self {
    parse_res
  }

  pub fn register(&mut self, res: Self) -> (Self, Option<Node>) {
    // println!("register parser result: {:#?}", res);
    match res.error.clone() {
      Some(err) => {
        self.error = res.error.clone();
      }

      None => {}
    }

    (res.clone(), res.node.clone())
  }
  pub fn success(
    &mut self,
    node: Option<Node>,
    line_fn_arg_vec: Vec<Option<Vec<TokenType>>>,
    ast: Option<(usize, Self)>,
  ) -> Self {
    // self.line_fn_arg_vec = line_fn_arg_vec;
    self.node = node;
    println!("huh should be success in parse res {:#?}", ast);
    match ast.clone() {
      Some((idx, pr)) => {
        FN_HASH.lock().unwrap().insert(idx, pr);

        println!(
          "alright buddy now we have the self after the parse line fn hash insert {:#?}",
          self
        );
      }
      None => {}
    };
    self.clone()
  }
  pub fn failure(&mut self, error: Error) -> Self {
    // println!("passwed in parseres failure {:#?}", error);
    self.error = Some(error);
    // println!("ok error should have changed {:#?}", self.error);
    let a = self.clone();
    // println!("did it really though {:#?}", a);
    a
  }
}

/* #[derive(Clone, Debug)]
pub struct SymbolTable {
  pub symbol_hash: HashMap<String, Token>,
  pub parent: Option<Box<SymbolTable>>,
}

impl Default for SymbolTable {
  fn default() -> Self {
    SymbolTable {
      symbol_hash: HashMap::new(),
      parent: None,
    }
  }
}
impl SymbolTable {
  fn get(&self, name: String) -> Option<Token> {
    let val = match self.clone().symbol_hash.clone().get(&name.clone()).clone() {
      Some(_) => Some(self.clone().symbol_hash.clone()[&name.clone()].clone()),
      None => None,
    };
    if val.is_none() && self.parent.clone().is_some() {
      let par = self.parent.clone().unwrap();
      // return self.parent.clone().unwrap().symbol_hash.clone().get(&name.clone());
      return match par.symbol_hash.clone().get(&name.clone()) {
        Some(_) => Some(par.symbol_hash.clone()[&name.clone()].clone()),
        None => None,
      };
    }

    return val;
  }

  fn set(&mut self, name: String, value: Token) {
    self.symbol_hash.insert(name, value);
  }

  fn remove(&mut self, name: String) {
    self.symbol_hash.remove(&name);
  }
} */
