use std::io::{BufRead, Write};
use crate::interpreter::{Interpreter, Node};

pub struct REPL {
    interpreter: Interpreter,
}

impl REPL {
    pub fn new() -> REPL {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    pub fn begin(&self) {
        println!("\nLISP Interpreter V0.1.0");
    }

    pub fn prompt(&mut self) {
        print!("\n=> ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line).unwrap();

        // NOTE: The interpreter should NOT reset every time since REPL commands are additive.
        self.interpreter.reset();
        self.interpreter.src = line;
        self.interpreter.tokenize();
        self.interpreter.parse();
        self.interpreter.evaluate();

        // match &self.interpreter.res {
        //     Node::Symbol(s) => println!("{:?}", s),
        //     Node::Integer(i) => println!("{:?}", i),
        //     Node::List(nodes) => println!("{:?}", nodes),
        //     _ => {}
        // }

        println!("{:?}", &self.interpreter.res);
    }
}
