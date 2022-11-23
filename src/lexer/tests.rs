use crate::lexer::{functions::get_tokens, token::Token};

#[test]
fn test_next_token() {
    let input = "let five = 5;";

    let case = [
        Token::Let,
        Token::Identifier("five".to_string()),
        Token::Assign,
        Token::Int(5),
        Token::Semicolon,
    ];
    let tokens = get_tokens(input);
    assert!(case.iter().zip(tokens).all(|(a, b)| *a == b))
}
