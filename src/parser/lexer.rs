use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Byte(u8),
    Char(char),
    Star,
    Plus,
    Question,
    Dot,
    LParen,
    RParen,
    Alt,
    EOF,
}

impl Token {
    pub fn is_atom_start(&self) -> bool {
        matches!(self, Token::Byte(_) | Token::Dot | Token::LParen)
    }
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
            Some(c) if c.is_ascii_alphanumeric() => Token::Byte(c as u8),
            Some(c) if c.is_alphanumeric() => Token::Char(c),
            Some('*') => Token::Star,
            Some('+') => Token::Plus,
            Some('?') => Token::Question,
            Some('.') => Token::Dot,
            Some('|') => Token::Alt,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            None => Token::EOF,
            Some(other) => panic!("Unknown character: {}", other),
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    fn lex_all(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            tokens.push(tok.clone());
            if tok == Token::EOF {
                break;
            }
        }
        tokens
    }

    #[test]
    fn single_token_tests() {
        assert_eq!(lex_all("a"), vec![Token::Byte(b'a'), Token::EOF]);
        assert_eq!(lex_all("*"), vec![Token::Star, Token::EOF]);
        assert_eq!(lex_all("|"), vec![Token::Alt, Token::EOF]);
        assert_eq!(lex_all("("), vec![Token::LParen, Token::EOF]);
        assert_eq!(lex_all(")"), vec![Token::RParen, Token::EOF]);
        assert_eq!(lex_all("+"), vec![Token::Plus, Token::EOF]);
        assert_eq!(lex_all("?"), vec![Token::Question, Token::EOF]);
        assert_eq!(lex_all("."), vec![Token::Dot, Token::EOF]);
    }

    #[test]
    fn multi_token_sequence() {
        assert_eq!(
            lex_all("a*b|(c)"),
            vec![
                Token::Byte(b'a'),
                Token::Star,
                Token::Byte(b'b'),
                Token::Alt,
                Token::LParen,
                Token::Byte(b'c'),
                Token::RParen,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(lex_all(""), vec![Token::EOF]);
    }

    #[test]
    #[should_panic(expected = "Unknown character: &")]
    fn invalid_character_ampersand() {
        let mut lexer = Lexer::new("a&");
        lexer.next_token(); // 'a'
        lexer.next_token(); // should panic on '&'
    }

    #[test]
    #[should_panic(expected = "Unknown character: $")]
    fn invalid_character_dollar() {
        let mut lexer = Lexer::new("($)");
        lexer.next_token(); // '('
        lexer.next_token(); // should panic on '$'
    }

    #[test]
    fn edge_alphanumeric_characters() {
        assert_eq!(
            lex_all("azAZ019"),
            vec![
                Token::Byte(b'a'),
                Token::Byte(b'z'),
                Token::Byte(b'A'),
                Token::Byte(b'Z'),
                Token::Byte(b'0'),
                Token::Byte(b'1'),
                Token::Byte(b'9'),
                Token::EOF
            ]
        );
    }

    #[test]
    fn repeated_operators_and_groups() {
        assert_eq!(
            lex_all("((aa*)|(bb))*"),
            vec![
                Token::LParen,
                Token::LParen,
                Token::Byte(b'a'),
                Token::Byte(b'a'),
                Token::Star,
                Token::RParen,
                Token::Alt,
                Token::LParen,
                Token::Byte(b'b'),
                Token::Byte(b'b'),
                Token::RParen,
                Token::RParen,
                Token::Star,
                Token::EOF
            ]
        );
    }

    #[test]
    fn unicode_alphanumerics_are_accepted() {
        let input = "λπЖ9";
        assert_eq!(
            lex_all(input),
            vec![
                Token::Char('λ'),
                Token::Char('π'),
                Token::Char('Ж'),
                Token::Byte(b'9'),
                Token::EOF
            ]
        );
    }
}
