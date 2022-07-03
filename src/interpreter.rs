use crate::context::*;
use crate::error::*;
use crate::node::*;
use crate::parser::*;
use crate::position::*;
use crate::token::*;
use crate::value::*;
// use crate::::*;

pub struct Interpreter {}

impl Interpreter {
  pub fn visit(&self, node: Node, ctx: Context) -> RTResult {
    match node.clone() {
      Node::BinOpNode {
        box left_node,
        box right_node,
        op_tok,
        pos_start,
        pos_end,
      } => self.visit_binop_node(left_node, right_node, op_tok, pos_start, pos_end, ctx),
      Node::NumberNode {
        tok,
        pos_start,
        pos_end,
      } => self.visit_number_node(tok, pos_start, pos_end, ctx),
      _ => unreachable!(),
    }
  }

  fn visit_number_node(
    &self,
    tok: Token,
    pos_start: Option<Position>,
    pos_end: Option<Position>,
    ctx: Context,
  ) -> RTResult {
    println!("visit number node tok: {:#?}", tok);
    let e = (RTResult {
      value: None,
      error: None,
    })
    .success(Some(
      (Number {
        value: tok.value.unwrap(),
        pos_start: pos_start,
        pos_end: pos_end,
        context: Some(ctx),
      }), // .set_context(Some(ctx))
          // .set_pos(pos_start, pos_end),
    ));
    // println!("{:#?}", e);
    e
  }
  fn visit_binop_node(
    &self,
    left_node: Node,
    right_node: Node,
    op_tok: Token,
    pos_start: Option<Position>,
    pos_end: Option<Position>,
    ctx: Context,
  ) -> RTResult {
    println!("visit binop node");
    let mut res = RTResult {
      value: None,
      error: None,
    };
    let mut (_, left) = res.register(self.visit(left_node.clone(), ctx.clone()));
    match res.error.clone() {
      Some(e) => return res.clone(),
      None => {}
    };
    // if res.err
    let mut (_, right) = res.register(self.visit(right_node.clone(), ctx.clone()));
    match res.error.clone() {
      Some(e) => return res.clone(),
      None => {}
    };
    let left = left.unwrap();
    let right = right.unwrap();
    let mut result = match op_tok.typ {
      TokenType::PLUS => left.add(right),
      TokenType::MUL => left.mul(right),
      TokenType::DIV => left.div(right),
      _ => unreachable!(),
    };

    match result.clone() {
      Ok(mut rs) => res.success(Some(rs.set_pos(pos_start, pos_end))),
      Err(e) => res.failure(Some(e)),
    }
  }}
    // if op_tok ==

    // println!("visit binop node")
  }
  /* fn visit_number_node() {

  } */

  /* fn no_visit_method(&self, node: Node) {

  } */
}
