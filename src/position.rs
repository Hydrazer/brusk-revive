#[derive(Debug, Clone, PartialEq)]
pub struct Position {
  pub filen: String,
  pub ftxt: String,
  pub idx: isize,
  pub ln: isize,
  pub col: isize,
}

impl Default for Position {
  fn default() -> Self {
    Position {
      idx: -1,
      ln: 0,
      col: -1,
      filen: "".to_string(),
      ftxt: "".to_string(),
    }
  }
}

impl Position {
  pub fn new(pos: Position) -> Self {
    pos
  }

  pub fn advance(&mut self, current_char: Option<char>) -> Position {
    self.idx += 1;
    self.col += 1;

    if current_char == Some('\n') {
      self.col = 0;
      self.ln += 1;
    }

    self.clone()
  }

  pub fn copy(&self) -> Position {
    Position {
      idx: self.idx,
      ln: self.ln,
      col: self.col,
      filen: self.filen.clone(),
      ftxt: self.ftxt.clone(),
    }
  }
}
