pub use super::token_type::TokenType;

pub struct Token {
    pub line: i32,
    pub tok_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(line: i32, tok_type: TokenType, value: String) -> Token {
        Token {
            line,
            tok_type,
            value,
        }
    }
}