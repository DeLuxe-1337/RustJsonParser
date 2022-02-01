pub use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub line: i32,
    pub tok_type: TokenType,
    pub value: String,
}