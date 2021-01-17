use std::io;
use std::io::*;
use crate::lexer::Lexer;

pub struct Repl;

impl Repl {
    const PROMT: &'static str = "λ >> ";

    pub const fn new() -> Repl {
        Repl
    }

    pub fn run(&mut self) {
        let lexer = Lexer::new();
        loop {
            print!("{}", Repl::PROMT);
            let mut buffer = String::new();

            let stdin = io::stdin();
            let mut stdout = io::stdout();

            stdout.flush().unwrap();
            stdin.read_line(&mut buffer).unwrap();

            lexer.tokenize(&buffer).for_each(|token| println!("{:?}", token));
            buffer.clear();
        }
    }
}
