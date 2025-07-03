use std::{env, fs};
use lisp::repl::REPL;

fn main() {
    let mut repl = REPL::new();

    repl.begin();

    loop {
        repl.prompt();
    }
}
