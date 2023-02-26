mod ast;
mod eval;
mod lexer;
mod parser;

use eval::NumericObject;
use std::{
    collections::HashMap,
    io::{self, Write},
};

fn start_repl() {
    let mut evaluation_env: HashMap<String, NumericObject> = HashMap::new();
    println!(">> Alang REPL started, have fun!!");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        let line: String = line.trim().into();

        if bytes_read > 0 && line.len() > 0 {
            match lexer::extract_token_stream(line) {
                Ok(tokens) => {
                    let exp_tree = parser::parse(tokens).unwrap();
                    eval::evaluate(exp_tree, &mut evaluation_env);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}

fn main() {
    start_repl()
}
