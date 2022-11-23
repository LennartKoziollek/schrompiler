// use scanner::generate_tokens;

use std::fs;

mod scanner;
mod parser;

fn main() {

    let contents = fs::read_to_string("./test.schromp").expect("cant open file");
    let tokens = scanner::scanner(contents);
    println!("{:?}", tokens);

}
