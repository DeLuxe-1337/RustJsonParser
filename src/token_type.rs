#[derive(Eq, PartialEq)]
#[derive(Clone, Copy)]
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