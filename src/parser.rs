use super::token::Token;
use super::token_type::TokenType;
use super::node::{Node, ObjectType};

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
            //println!("Block");
            let mut nodes = Vec::new();

            while !self.check(TokenType::BlockEnd) && !self.is_end() {
                nodes.push(self.block());
            }

            self.consume(TokenType::BlockEnd, "Expected '}'");

            if self.peek().tok_type == TokenType::Comma {
                //println!("Comma consumed");
                self.consume(TokenType::Comma, "Expected ',' if you want more children");
            }

            let body = Box::new(nodes);

            return Node::BlockNode(body);
        }
        
        return self.assignment();
    }

    fn assignment(&mut self) -> Node {
        if self.check(TokenType::String) && self.peek_c(1).tok_type == TokenType::Colon {
            //println!("Assignment");
            let name = self.consume(TokenType::String, "Expected string").clone();
            self.advance();
            let init = self.block();

            return Node::AssignmentNode(name.value,  Box::new(init));
        }
        
        return self.values();
    }

    fn values(&mut self) -> Node {
        let array = [TokenType::String, TokenType::Number, TokenType::False, TokenType::True, TokenType::Null];
        if self.match_abund(array.to_vec()) {
            //println!("Value");

            let previous = self.previous().clone();

            if self.peek().tok_type != TokenType::BlockEnd {
                self.consume(TokenType::Comma, "Expected ',', if you want more children");
            }

            let mut ot: ObjectType = ObjectType::Null;

            match previous.tok_type {
                TokenType::String => ot = ObjectType::String(previous.value.clone()),
                TokenType::Number => ot = ObjectType::Number(previous.value.parse::<i32>().unwrap()),
                TokenType::True => ot = ObjectType::Bool(true),
                TokenType::False => ot = ObjectType::Bool(false),
                TokenType::Null => ot = ObjectType::Null,
                _ => panic!("Unknown token type"),
            }

            return Node::ValueNode(previous.value, ot);
        }

        //println!("{}", self.tokens[self.current].value);
        panic!("End of parser reached");
    } 

    pub fn parse(&mut self) {
        while !self.is_end() {
            let node = self.block();
            self.nodes.push(node);
        }
    }
}