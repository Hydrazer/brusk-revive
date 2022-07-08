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

  ListNode {
    tok_vec: Vec<Token>,
    pos_start: Option<Position>,
    pos_end: Option<Position>,
  },
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
      Node::ListNode { tok_vec, .. } => {
        write!(f, "{:?}", tok_vec)
      }
    }
  }
}
