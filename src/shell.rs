use std::env;
use std::fs;
use std::io::{self, Write};
use crate::basic;
use crate::transpile::*;
use std::process::Command;

pub fn run() {
  let file_name = env::args().nth(1).expect("No file passed in");
  let env_arg_vec = env::args().skip(2).collect::<Vec<_>>();
  let mut code = fs::read_to_string(file_name.clone())
    .expect("Unable to read file")
    .split("\n")
    .filter(|s| !s.starts_with("#"))
    .collect::<Vec<_>>()
    .join("\n");

  while code.ends_with("\n") {
    code.pop();
  }

  code += "\n";
  let res = basic::run(code.clone(), file_name, env_arg_vec);
  match res {
    Ok((arg_vec, r)) => {
      println!("correcto {:#?}", r);
      let mut rust_code = Transpile {
        parse_line_fn_hash: r,
        env_arg_ind: -1,
      };

      let mut f = fs::File::create("bruh.brusk").expect("Unable to create file");
      let transpiled_code = rust_code.transpile(arg_vec);
      f.write_all(transpiled_code.clone().as_bytes())
        .expect("unable to write to file");

      let mut proc = Command::new("./nice.bash");
      println!("{}", transpiled_code);
      println!("{:#?}", proc.output());

    }

    Err(e) => {
      println!("{e:#?}");
      std::process::exit(69);
    }
  }
}

#[macro_export]
macro_rules! input {
  () => {{
    input!(String)
  }};

  ($t:ty) => {{
    let input = &mut "".into();
    std::io::stdin().read_line(input).unwrap();
    if input.ends_with("\n") {
      input.pop();
    };
    input.parse::<$t>().unwrap()
  }};
}

pub(crate) use input;
