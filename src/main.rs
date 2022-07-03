#![feature(box_patterns)]
mod basic;
// mod context;
mod error;
// mod interpreter;
mod lexer;
mod node;
mod parser;
mod position;
mod shell;
mod token;
mod transpile;
// mod value;

fn main() {
  shell::run()
}
