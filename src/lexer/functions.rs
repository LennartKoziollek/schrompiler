use super::token::Token;

///
/// Takes code as input and generates Tokens out of it
///
pub fn get_tokens(code: &str) -> Vec<Token> {
    let mut char_indices = code.char_indices().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some((position, char)) = char_indices.next() {
        //loop through code until end
        let token = match char {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            // eq or assign
            '=' => match char_indices.next_if_eq(&(position + 1, '=')) {
                Some(_equals) => Token::Eq,
                None => Token::Assign,
            },
            // not equal
            '!' => match char_indices.next_if_eq(&(position + 1, '=')) {
                Some(_equals) => Token::NotEq,
                None => Token::Bang,
            },
            // numbers
            // TODO no whitespace characters behind number possible
            c if char::is_numeric(c) => {
                let mut last_matched: char = '\0';
                let number_vec: Vec<u32> = char_indices
                    .by_ref()
                    .take_while(|(_pos, c)| {
                        last_matched = *c;
                        c.is_numeric()
                    })
                    .map(|(_pos, c)| c.to_digit(10).unwrap())
                    .collect();

                let mut ret_number: u32 = c.to_digit(10).unwrap();
                let mut factor: u32 = 10;

                number_vec.iter().rev().for_each(|x| {
                    ret_number = ret_number + x * factor;
                    factor = factor * 10;
                });

                Token::Int(ret_number)
            }
            c if char::is_alphabetic(c) => {
                let first = c.to_string();
                let mut last_matched: char = '\0';
                let mut s: String = char_indices
                    .by_ref()
                    .take_while(|(_pos, c)| {
                        last_matched = *c;
                        c.is_alphabetic()
                    })
                    .map(|(_pos, c)| c)
                    .collect();

                s.insert_str(0, &first);
                match s.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    _ => Token::Identifier(s),
                }

            }
            // string literal
            '"' => {
                let mut last_matched: char = '\0';
                let s: String = char_indices
                    .by_ref()
                    .take_while(|(_pos, c)| {
                        last_matched = *c;
                        *c != '"'
                    })
                    .map(|(_pos, c)| c)
                    .collect();

                match last_matched {
                    '"' => Token::String(s),
                    _ => Token::Illegal,
                }
            }
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBraces,
            '}' => Token::RBraces,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            //ignore whitespace
            ' ' => continue,
            '\n' => continue,
            _ => Token::Illegal,
        };
        tokens.push(token);
    }

    tokens
}
