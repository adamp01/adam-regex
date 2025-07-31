use crate::lexer::{Lexer, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Regex {
    Char(char),
    Star(Box<Regex>),
    Concat(Box<Regex>, Box<Regex>),
    Alt(Box<Regex>, Box<Regex>),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current = lexer.next_token();
        Self { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Regex {
        self.parse_alt()
    }

    pub fn parse_alt(&mut self) -> Regex {
        let mut expr = self.parse_concat();

        while self.current == Token::Alt {
            self.advance();
            let right = self.parse_concat();
            expr = Regex::Alt(Box::new(expr), Box::new(right));
        }

        expr
    }

    fn parse_concat(&mut self) -> Regex {
        let mut expr = self.parse_star();

        while matches!(self.current, Token::Char(_) | Token::LParen) {
            let right = self.parse_star();
            expr = Regex::Concat(Box::new(expr), Box::new(right));
        }

        expr
    }

    fn parse_star(&mut self) -> Regex {
        let mut expr = self.parse_atom();

        while self.current == Token::Star {
            self.advance();
            expr = Regex::Star(Box::new(expr));
        }

        expr
    }

    fn parse_atom(&mut self) -> Regex {
        match &self.current {
            Token::Char(c) => {
                let node = Regex::Char(*c);
                self.advance();
                node
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_alt();
                if self.current != Token::RParen {
                    panic!("Expected ')'");
                }
                self.advance();
                expr
            }
            _ => panic!("Unexpected token: {:?}", self.current),
        }
    }
}

pub fn parse(input: &str) -> Regex {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse()
}
