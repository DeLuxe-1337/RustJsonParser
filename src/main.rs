mod token;
mod token_type;
mod node;

mod lexer;
mod parser;
mod japi;

use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();

    let source = std::fs::read_to_string("D:\\Rust\\JsonParser\\src\\my_json.json").expect("Expected file");

    let mut lexer = lexer::lexer_t::new(source.as_str());
    lexer.scan_tokens();

    for token in lexer.tokens.clone() {
        println!("Token: {:?}, {1}, {2}", token.tok_type, token.value, token.line);
    }

    let mut parser = parser::parser_t::new(lexer.tokens);
    parser.parse();

    for node in parser.nodes.clone() {
        println!("{:?}", node);
    }

    let mut api = japi::japi_t::new(parser.nodes.clone());
    let res = api.index("test>deez>nested");

    println!("result: {:?}", res);

    println!("It took {} ms to run!", now.elapsed().as_millis());

    std::io::stdin().read_line(&mut String::new());
}
