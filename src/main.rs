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

    let mut api = japi::japi_t::new(source);
    
    //println!("Type the path you want to visit");
    //let mut input = String::new();
    //std::io::stdin().read_line(&mut input);

    //let res = api.index(&input);
    let res = api.index("data>id");

    println!("result: {:?}", res);

    println!("It took {} ms to run!", now.elapsed().as_millis());

    std::io::stdin().read_line(&mut String::new());
}
