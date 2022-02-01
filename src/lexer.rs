use super::token::Token;
use super::token_type::TokenType;

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

pub struct lexer_t {
    input: String,
    start: i32,
    current: i32,
    line: i32,

    pub tokens: Vec<Token>,
    chars: Vec<char>,
}

impl lexer_t {
    pub fn new(source: &str) -> lexer_t {
        return lexer_t {
            input: source.to_string(),
            start: -1,
            current: -1,
            line: 1,
            tokens: Vec::new(),
            chars: source.chars().collect(),
        };
    }

    fn is_end(&mut self) -> bool {
        return self.current >= (self.input.len() - 1) as i32;
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.chars[self.current as usize];
    }

    fn peek(&mut self) -> char {
        return self.chars[(self.current + 1) as usize];
    }

    fn peek_c(&mut self, idx: i32) -> char {
        return self.chars[(self.current + idx) as usize];
    }
    fn add_token(&mut self, tok_type: TokenType, value: String) {
        self.tokens.push(Token {
            line: self.line,
            tok_type: tok_type,
            value: value,
        });
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_end() {
            println!("Unterminated string");
            return;
        }

        self.advance();

        let value = self.input.substring(
            (self.start + 1) as usize,
            ((self.current - 1) - (self.start - 1)) as usize,
        );

        self.add_token(TokenType::String, value);
    }

    fn number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_c(2).is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let value = self.input.substring(
            (self.start + 1) as usize,
            ((self.current - 1) - (self.start - 1)) as usize,
        );

        self.add_token(TokenType::Number, value);
    }

    fn data_type(&mut self) {
        while self.peek().is_alphabetic() {
            self.advance();
        }

        let value = self.input.substring(
            (self.start + 1) as usize,
            ((self.current - 1) - (self.start - 1)) as usize,
        );

        match value.as_str() {
            "false" => self.add_token(TokenType::False, value),
            "true" => self.add_token(TokenType::True, value),
            "null" => self.add_token(TokenType::Null, value),
            _ => {}
        }
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '{' | '[' => self.add_token(TokenType::BlockStart, String::from(c)),
            '}' | ']' => self.add_token(TokenType::BlockEnd, String::from(c)),

            ',' => self.add_token(TokenType::Comma, String::from(c)),
            ':' => self.add_token(TokenType::Colon, String::from(c)),

            ' ' => {}
            '\n' => self.line += 1,
            '"' => self.string(),

            _ => {
                if c.is_numeric() {
                    self.number();
                } else if c.is_alphabetic() {
                    self.data_type();
                }
            }
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::End, "".to_string());
    }
}
