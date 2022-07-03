use crate::position::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Context {
  pub display_name: String,
  pub parent: Option<Box<Context>>,
  pub parent_entry_pos: Option<Position>,
}

/* impl Context {
    fn
} */
