mod token;
mod token_type;
mod node;

use node::ObjectType;

mod lexer;
mod parser;
mod japi;

use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();

    let source = std::fs::read_to_string("D:\\Rust\\JsonParser\\src\\my_json.json").expect("Expected file");

    let api = japi::japi_t::new(source);

    //println!("Type the path you want to visit");
    //let mut input = String::new();
    //std::io::stdin().read_line(&mut input);

    //let res = api.index(&input);
    let result = api.index("data>id");

    println!("result: {0}", result.as_int() + 100);

    println!("It took {} ms to run!", now.elapsed().as_millis());

    std::io::stdin().read_line(&mut String::new());
}