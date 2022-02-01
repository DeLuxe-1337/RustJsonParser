use super::token::Token;
use super::token_type::TokenType;
use super::node::Node;
#[derive(std::clone::Clone)]

pub struct parser_t {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub nodes: Vec<Node>,
}

impl parser_t {
    pub fn new(tokens: Vec<Token>) -> parser_t {
        return parser_t {
            tokens: tokens,
            current: 0,
            nodes: Vec::new(),
        };
    }

    fn is_end(&self) -> bool {
        return self.peek().tok_type == TokenType::End;
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn peek_c(&self, count: i32) -> &Token {
        return &self.tokens[self.current + count as usize];
    }

    fn check(&self, t: TokenType) -> bool {
        return self.peek().tok_type == t;
    }

    fn match_tok(&mut self, tok_type: TokenType) -> bool {
        if self.check(tok_type) {
            self.advance();
            return true;
        }

        return false;
    }

    fn match_abund(&mut self, toks: Vec<TokenType>) -> bool {
        for tok in toks {
            if self.match_tok(tok) == true {
                return true;
            }
        }

        return false;
    }
    
    fn consume(&mut self, tok_type: TokenType, error: &str) -> &Token {
        if self.check(tok_type) {
            return self.advance();
        }

        panic!("Error has occured {}", error.to_string());
    }

    fn advance(&mut self) -> &Token {
        if !self.is_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn block(&mut self) -> Node {
        if self.match_tok(TokenType::BlockStart) {
            println!("Block");
            let mut nodes = Vec::new();

            while !self.check(TokenType::BlockEnd) && !self.is_end() {
                nodes.push(self.block());
            }

            self.consume(TokenType::BlockEnd, "Expected '}'");

            if self.peek().tok_type == TokenType::Comma && (self.match_tok(TokenType::BlockStart) || self.match_tok(TokenType::String)) {
                self.consume(TokenType::Comma, "Expected ',' if you want more children");
            }

            let body: Option<Box<Vec<Node>>> = Option::from(Box::new(nodes));

            return Node::BlockNode {
                statements: body,
            };
        }
        
        return self.assignment();
    }

    fn assignment(&mut self) -> Node {
        if self.check(TokenType::String) && self.peek_c(1).tok_type == TokenType::Colon {
            println!("Assignment");
            let name = self.consume(TokenType::String, "Expected string").clone();
            self.advance();
            let init = self.block();

            return Node::AssignmentNode {
                name: name.value,
                value: Option::from(Box::new(init)),
            };
        }
        
        return self.values();
    }

    fn values(&mut self) -> Node {
        let array = [TokenType::String, TokenType::Number, TokenType::False, TokenType::True, TokenType::Null];
        if self.match_abund(array.to_vec()) {
            println!("Value");
            return Node::ValueNode {
                value: self.previous().value.clone(),
            };
        }

        println!("{}", self.tokens[self.current].value);
        panic!("End of parser reached");
    } 

    pub fn parse(&mut self) {
        while !self.is_end() {
            let node = self.block();
            self.nodes.push(node);
        }
    }
}