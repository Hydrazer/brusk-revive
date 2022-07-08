use crate::node::*;
use crate::basic::*;
use crate::parser::*;
use crate::token::*;
use std::collections::HashMap;

const FUNC_STR: &'static str = "
fn negate(a: f64) -> f64 {
  -a
}
fn plus(a: f64, b:f64) -> f64 {
  a + b
}

fn mul(a: f64, b:f64) -> f64 {
  a * b
}

fn div(a: f64, b:f64) -> f64 {
  a / b
}

fn modulus(a: f64, b:f64) -> f64 {
  a % b
}
fn sub(a: f64, b:f64) -> f64 {
  a - b
}

fn map(a: &dyn Fn(f64) -> f64, b: Vec<f64>) -> Vec<f64> {
  b.clone().into_iter().map(|x| a(x)).collect()
}

#[macro_export]
macro_rules! cons {
  ($x:expr, $l:expr) => {{
    let mut l = $l;
    l.insert(0, $x);
    l
  }};
}
";

pub struct Transpile {
  pub parse_line_fn_hash: HashMap<usize, Ast>,
  pub env_arg_ind: isize,
  /* pub env_arg_vec: Vec<Token>,
  pub env_arg_ind: isize, */
  /* token_vec: Vec<Token>,
  line_fn_arg_vec: Vec<Option<Vec<Option>>>,
  line_fn_arg_vec: Vec<Option<Vec<Option>>>, */
  // line_fn_arg_vec: Vec<Token>,
  // line_fn_arg_vec: Vec<Token>,
}

impl Transpile {
  pub fn transpile(&mut self, arg_vec: Vec<Token>) -> String {
    /* let mut hv = self.parse_line_fn_hash.into_iter().collect::<Vec<_>>();
    hv.sort_by_key(|a| a.0); */
    let mut out_vec = (0..self.parse_line_fn_hash.len())
      .map(|_| "".to_string())
      .collect::<Vec<_>>();
    for (k, v) in self.parse_line_fn_hash.clone() {
      let node = v.node.unwrap();
      println!("what on earth this deez nuts node ||  {:#?}", node);
      // matc
      let code = format!(
        "fn line{}({}) -> {} {{
  {}
}}",
        k,
        // "".to_string(),
        /* match  {
        Some(l) => { */
        arg_vec
          .clone()
          .into_iter()
          .enumerate()
          .map(|(idx, v)| format!("arg_{idx}: {}", v.typ.rust_typ()))
          .collect::<Vec<_>>()
          .join(", "),
        /* }
            None => "".to_string()
        },  */
        match node.clone() {
            Node::ListNode {..} => {
                "Vec<f64>"
            }
            Node::NumberNode {..} => {
                "f64"
            }
            Node::BinOpNode {op_tok, ..} => {
                match op_tok.typ.clone() {
                    TokenType::LIST_EMPTY => {
                        "Vec<f64>"
                    }
                    TokenType::CONS | TokenType::MAP => {
                        "Vec<f64>"
                    }
                    _ => "f64"
                }
                // "f64"
            }
            _ => unreachable!()
        },
        self.transpile_node(node.clone(), 0, false)
      );
      /* (match v.line_fn_arg_vec[k].clone() {
          Some(l) => {
            l.into_iter().enumerate().map(|(idx, v)| => format!("arg_{idx}: {}", v.typ.rust_typ())).collect::<Vec<_>>().join(", ")
          }
          None => "".to_string()
      }),

      self.transpile_node(node.clone())); */
      out_vec[k] = code;
      // println!("node {:#?} || line fn arg vec {:#?}\n\n", node, v.line_fn_arg_vec[k]);
    }
    format!(
      r###"fn main() {{
  println!("{{:?}}", line0({}));
}}
{}
{}"###,
      arg_vec
        .into_iter()
        .map(|x| format!(
          "{:?}_f64",
          match x.value.unwrap() {
            TokenVal::FLOAT(x) => {
              x
            }
            _ => unreachable!(),
          }
        ))
        .collect::<Vec<_>>()
        .join(", "),
      out_vec.join("\n\n"),
      FUNC_STR
    )

    // "".to_string()
  }

  pub fn transpile_node(&mut self, node: Node, depth: usize, is_map_first: bool) -> String {
    match node.clone() {
      Node::BinOpNode {
        op_tok, arg_vec, ..
      } => {
        let mut depth_curr = depth;
        println!("binopnode");
        arg_vec
          .clone()
          .into_iter()
          .for_each(|a| println!("{:#?}", a));
        let tok_str = match op_tok.clone().typ {
          TokenType::PLUS => "plus",
          TokenType::NEGATE => "negate",
          TokenType::MUL => "mul",
          TokenType::DIV => "div",
          TokenType::MINUS => "sub",
          TokenType::MOD => "modulus",
          TokenType::CONS => "cons!",
          TokenType::MAP => {depth_curr += 1;"map"},
          _ => unreachable!(),
        };
        println!("this is arg vec {:#?}", arg_vec);
        let fmter = format!(
          "{}({})",
          tok_str,
          arg_vec
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, a)| 
              // println!("btw a {:#?}", a);
            (match a {
                Some(a) => {
                    self.transpile_node(a, depth_curr, match (i, op_tok.clone().typ) {
                        (0, TokenType::MAP) => true,
                        _ => false
                    }) 
                } 
            None => {
                // self.env_arg_ind += 1;
                format!("arg_{}_{}", self.env_arg_ind, depth_curr)
            } 


        }))
            .collect::<Vec<_>>()
            .join(", ")
        );

        match is_map_first {
            false => fmter,
            _ => {format!("&|arg_0_{}| {}", depth_curr, fmter)}
        }
      }

      Node::NumberNode { tok, .. } => {
        println!(
          "curr tok in trans nn {:#?} ^*^ {:#?}",
          tok.clone(),
          tok.clone().value.unwrap().clone()
        );
        match tok.clone().typ.clone() {
          TokenType::FLOAT => format!(
            "{}_f64",
            match tok.value.clone().unwrap().clone() {
              TokenVal::FLOAT(x) => {
                x.to_string()
              }
              _ => unreachable!(),
            }
          ),
          TokenType::VAR => match tok.value.clone().unwrap().clone() {
            TokenVal::VAR(v) => {
              // self.env_arg_ind += 1;
             
              format!("arg_{}_{}", VAR_STR.chars().position(|c| c.to_string() == v).unwrap(), depth)
            }
            _ => unreachable!(),
          },
          _ => unreachable!(),
        }
        // println!("numbernode {:#?}", tok);
      }
      Node::ListNode {.. } => {
        format!("vec![]")
      }
      _ => unreachable!(),
      // Node::VarAccessNode {..}

    }
  }
}
