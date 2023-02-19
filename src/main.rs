mod ast;
mod eval;
mod lexer;
mod parser;

use std::io::{self, Write};

fn start_repl() {
    println!(">> Alang REPL started, have fun!!");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        if bytes_read > 0 {
            match lexer::extract_token_stream(line) {
                Ok(tokens) => {
                    let exp_tree = parser::parse(tokens).unwrap();
                    eval::evaluate(exp_tree);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}

fn main() {
    start_repl()
}
