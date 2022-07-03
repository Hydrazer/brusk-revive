use crate::node::*;
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
      // matc
      let code = format!(
        "fn line{}({}) -> f64 {{
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
        self.transpile_node(node.clone())
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

  pub fn transpile_node(&mut self, node: Node) -> String {
    match node.clone() {
      Node::BinOpNode {
        op_tok, arg_vec, ..
      } => {
        println!("binopnode");
        arg_vec
          .clone()
          .into_iter()
          .for_each(|a| println!("{:#?}", a));
        let tok_str = match op_tok.typ {
          TokenType::PLUS => "plus",
          TokenType::NEGATE => "negate",
          TokenType::MUL => "mul",
          TokenType::DIV => "div",
          TokenType::MINUS => "sub",
          TokenType::MOD => "modulus",
          _ => unreachable!(),
        };
        format!(
          "{}({})",
          tok_str,
          arg_vec
            .clone()
            .into_iter()
            .map(|a| 
              // println!("btw a {:#?}", a);
            (match a {
                Some(a) => {
                    self.transpile_node(a) 
                } 
            None => {
                self.env_arg_ind += 1;
                format!("arg_{}", self.env_arg_ind)
            } 


        }))
            .collect::<Vec<_>>()
            .join(", ")
        )
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
              self.env_arg_ind += 1;
              format!("arg_{}", self.env_arg_ind)
            }
            _ => unreachable!(),
          },
          _ => unreachable!(),
        }
        // println!("numbernode {:#?}", tok);
      }
      _ => unreachable!(),
      // Node::VarAccessNode {..}
    }
  }
}
