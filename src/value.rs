use crate::context::*;
use crate::error::*;
use crate::position::*;
use std::fmt;
// use crate::value::*;

#[derive(Clone, Debug)]
pub struct Number {
  pub value: f64,
  pub pos_start: Option<Position>,
  pub pos_end: Option<Position>,
  pub context: Option<Context>,
}

/* #[derive(Clone, Debug)]
pub struct Number {
  pub value: f64,
  pub pos_start: Option<Position>,
  pub pos_end: Option<Position>,
  pub context: Option<Context>,
} */


/* #[derive(Clone)]
pub enum Value {
  Number {
    value: f64,
    pos_start: Option<Position>,
    pos_end: Option<Position>,
    context: Option<Context>,
  },
} */
/* impl Default for Value {
  fn default() -> Self {
    Value::Number {
      value: 0.0,
      pos_start: Position {
        ..Default::default()
      },
      pos_end: Position {
        ..Default::default()
      },
    }
  }
} */

/* impl fmt::Debug for Number {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.value)
    // match self.clone() {
    // Value::Number { value, .. } => {
    // write!(f, "{}", value)
    // }

    // _ => unreachable!(),
    // }
  }
} */

impl Number {
  pub fn new(num: Number) -> Self {
    let mut n = num.clone();
    n.set_pos(None, None);
    n.set_context(None);
    n
    /* match num {
      Value::Number { .. } => {
        let mut n = num.clone();
        n.set_pos(None, None);
        n.set_context(None);
        n
      }

      _ => unreachable!(),
    } */
  }

  pub fn set_context(&mut self, context: Option<Context>) -> Number {
    self.context = context;
    self.clone()
  }

  pub fn set_pos(&mut self, pos_start: Option<Position>, pos_end: Option<Position>) -> Number {
    self.pos_start = pos_start;
    self.pos_end = pos_end;
    self.clone()
    /* match self {
      Value::Number {
        pos_start: ps,
        pos_end: pe,
        ..
      } => {
        *ps = pos_start;
        *pe = pos_end;
        self.clone()
      }

      _ => unreachable!(),
    } */
  }

  pub fn add(self, other: Number) -> Result<Number, Error> {
    let mut v = Number {
      value: self.value + other.value,
      pos_start: None,
      pos_end: None,
      context: self.context.clone(),
    };

    v.set_context(self.context);
    Ok(v.clone())
  }

  pub fn mul(self, other: Number) -> Result<Number, Error> {
    let mut v = Number {
      value: self.value + other.value,
      pos_start: None,
      pos_end: None,
      context: self.context.clone(),
    };

    v.set_context(self.context);
    Ok(v.clone())
  }

  pub fn div(self, other: Number) -> Result<Number, Error> {
    // println!("my man other in div {:#?}", other);
    if other.value == 0.0 {
      Err(Error {
        pos_start: other.pos_start,
        pos_end: other.pos_end,
        err: ErrorType::RT("Division by Zero".to_string(), self.context.unwrap()),
      })
    } else {
      let mut v = Number {
        value: self.value / other.value,
        pos_start: None,
        pos_end: None,
        context: self.context.clone(),
      };

      v.set_context(self.context);
      Ok(v.clone())
    }
    // _ => unreachable!(),
  }
}
