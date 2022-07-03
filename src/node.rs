use crate::position::*;
use crate::token::*;
use std::fmt;

#[derive(Clone)]
pub enum Node {
  BinOpNode {
    op_tok: Token,
    arg_vec: Vec<Option<Node>>,
    /* left_node: Box<Node>,
    right_node: Box<Node>,
    op_tok: Token, */
    pos_start: Option<Position>,
    pos_end: Option<Position>,
  },

  NumberNode {
    tok: Token,
    pos_start: Option<Position>,
    pos_end: Option<Position>,
  },
}

pub struct BinOpNode {
  pub op_tok: Token,
  pub arg_vec: Vec<Option<Node>>,
  /* left_node: Box<Node>,
  right_node: Box<Node>,
  op_tok: Token, */
  pub pos_start: Option<Position>,
  pub pos_end: Option<Position>,
}

impl fmt::Debug for Node {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Node::BinOpNode {
        /* left_node,
        right_node, */
        op_tok,
        arg_vec,
        ..
      } => {
        write!(
          f,
          "{:?} =>
  {:#?}",
          op_tok, arg_vec
        )
      }

      Node::NumberNode { tok, .. } => {
        write!(f, "{:?}", tok)
      }
    }
  }
}
