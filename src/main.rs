extern crate nix;

use std::io::BufRead;
use std::io::stdout;
use nix::unistd::fork;
use std::io::{self, Write};
use nix::sys::wait::waitpid;
use nix::unistd::ForkResult;
use nix::unistd::execvp;
use std::ffi::CString;

fn tokenize(s : &String) -> Vec<CString> {
  s.split_whitespace()
   .map(CString::new)
   .map(Result::unwrap)
   .collect::<Vec<CString>>()
}

fn execute(tokens : Vec<CString>) {
  match fork().unwrap() {
    ForkResult::Parent{child} => {
      waitpid(child, None).unwrap();
    }
    ForkResult::Child => {
      execvp(&tokens[0], &tokens).unwrap();
    }
  }
}


fn main() {
  let stdin = io::stdin();
  let mut line = String::new();

  loop {
    print!("# ");
    stdout().flush().unwrap();

    stdin.lock().read_line(&mut line).unwrap();

    let tokens = tokenize(&line);

    execute(tokens);

    line.clear();
  }
}
