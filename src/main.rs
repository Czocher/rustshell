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

fn execute<'a>(tokens : Vec<CString>) {
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
  loop {
    print!("# ");
    stdout().flush().unwrap();
    let lines = stdin.lock().lines().map(Result::unwrap);

    for line in lines {
      let tokens = tokenize(&line);
      execute(tokens);
    }
  }
}
