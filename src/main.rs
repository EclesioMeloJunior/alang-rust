mod ast;
mod lexer;
mod parser;

fn main() {
    loop {
        let mut line = String::new();
        let bytes_read = std::io::stdin().read_line(&mut line).unwrap();
        if bytes_read > 0 {
            match lexer::extract_token_stream(line) {
                Ok(tokens) => {
                    parser::parse(tokens).unwrap();
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
