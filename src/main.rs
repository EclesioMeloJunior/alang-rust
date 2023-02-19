mod ast;
mod eval;
mod lexer;
mod parser;

fn main() {
    loop {
        let mut line = String::new();
        let bytes_read = std::io::stdin().read_line(&mut line).unwrap();
        if bytes_read > 0 {
            match lexer::extract_token_stream(line) {
                Ok(tokens) => {
                    let exp_tree = parser::parse(tokens).unwrap();
                    eval::evaluate(exp_tree);
                    //println!("{:?}", exp_tree);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
