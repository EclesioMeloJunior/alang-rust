mod ast;
mod eval;
mod lexer;
mod parser;

use eval::NumericObject;
use rustyline::{DefaultEditor, Result};
use std::collections::HashMap;

fn start_repl() -> Result<()> {
    let mut evaluation_env: HashMap<String, NumericObject> = HashMap::new();
    println!(">> Alang REPL started, have fun!!");

    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match lexer::extract_token_stream(line) {
                Ok(tokens) => {
                    let exp_tree = parser::parse(tokens).unwrap();
                    eval::evaluate(exp_tree, &mut evaluation_env);
                }
                Err(e) => println!("{:?}", e),
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    start_repl()
}
