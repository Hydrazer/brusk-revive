// use crate::context::*;
use crate::position::*;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Error {
  pub err: ErrorType,
  pub pos_start: Option<Position>,
  pub pos_end: Option<Position>,
}

#[derive(Clone, PartialEq)]
pub enum ErrorType {
  IllegalChar(String),
  IllegalSyntax(String),
  // RT(String, Context),
}

impl fmt::Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let (err_name, det) = match self.err.clone() {
      ErrorType::IllegalChar(det) => ("Illegal Character", det),
      ErrorType::IllegalSyntax(det) => ("Illegal Syntax", det),
    };

    write!(
      f,
      "{}: {}
ppFile: {}, line {}, idx {:?}",
      err_name,
      det,
      match self.pos_start.clone() {
        Some(ps) => ps.filen,
        None =>
          "wait how did you not pass in the pos start please tell Hydrazer they are nob".to_string(),
      },
      match self.pos_end.clone() {
        Some(pe) => pe.ln,
        None => 69420,
      },
      match self.pos_end.clone() {
        Some(ps) => ps.col,
        None => 80085,
      },
    )
    /* match self.err.clone() {
        ErrorType::RT(_) =>
    } */
  }
}

