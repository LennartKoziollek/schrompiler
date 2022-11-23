/*
*
* This is a super basic scanner that is not exhaustive enough for to be extended
*
* TODO use a formal system to parse text to tokens
*
*/

#[derive(Debug)]
pub enum Token {
    Plus,
    Equals,
    EqualsEquals,
    NotEquals,
    StringLiteral(String),
    If,
    OpenBracket,
    CloseBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Invalid(String),
}

///
/// Takes code as input and generates Tokens out of it
///
pub fn scanner(code: String) -> Vec<Token> {
    let mut char_indices = code.char_indices().peekable();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some((position, char)) = char_indices.next() {
        //loop through code until end
        let token = match char {
            '+' => Token::Plus,
            // equal or equalequal
            '=' => match char_indices.next_if_eq(&(position + 1, '=')) {
                Some(_equals) => Token::EqualsEquals,
                None => Token::Equals,
            },
            // not equal
            '!' => match char_indices.next_if_eq(&(position + 1, '=')) {
                Some(_equals) => Token::NotEquals,
                None => Token::Invalid("! is not a valid token".to_string()),
            },
            // numbers
            // '"' => {}
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
                    '"' => Token::StringLiteral(s),
                    _ => Token::Invalid("unterminated literal".to_string()),
                }
            }
            // identifier: if
            'i' => match char_indices.next_if_eq(&(position + 1, 'f')) {
                Some(_if) => Token::If,
                None => Token::Invalid("do you mean if?".to_string()),
            },
            '(' => Token::OpenBracket,
            ')' => Token::CloseBracket,
            '{' => Token::OpenCurlyBracket,
            '}' => Token::CloseCurlyBracket,
            //ignore whitespace
            ' ' => continue,
            '\n' => continue,
            _ => Token::Invalid("Invalid Character".to_string()),
        };
        tokens.push(token);
    }

    tokens
}
