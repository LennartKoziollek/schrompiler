use lexer::functions::get_tokens;

mod lexer;

fn main() {

    let input = "let five = 5 ;";

    let tokens = get_tokens(input); 

    println!("{:?}", tokens);
}
