#[derive(Debug)]
pub enum TokenType {
    String,
    Number,

    BlockStart,
    BlockEnd,

    Comma,
    Colon,

    False,
    True,
    Null,

    End,
}