use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Char(char),
    Star,
    LParen,
    RParen,
    Alt,
    EOF,
}

pub struct Lexer<'a> {
    input: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
       self.input.next()
    }

    pub fn next_token(&mut self) -> Token {
        match self.next_char() {
            Some(c) if c.is_alphanumeric() => Token::Char(c),
            Some('*') => Token::Star,
            Some('|') => Token::Alt,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            None => Token::EOF,
            Some(other) => panic!("Unknown character: {}", other),
        }
    }
}
