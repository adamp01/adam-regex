use crate::ast::Regex;
use crate::parser::lexer::{Lexer, Token};

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

        while matches!(self.current, Token::Byte(_) | Token::LParen) {
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
            Token::Byte(b) => {
                let node = Regex::Byte(*b);
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

#[cfg(test)]
mod parser_tests {
    use super::*;
    use crate::ast::Regex::{self, *};

    fn boxed(expr: Regex) -> Box<Regex> {
        Box::new(expr)
    }

    #[test]
    fn test_single_char() {
        let ast = parse("a");
        assert_eq!(ast, Byte(b'a'));
    }

    #[test]
    fn test_concat_two_chars() {
        let ast = parse("ab");
        assert_eq!(ast, Concat(boxed(Byte(b'a')), boxed(Byte(b'b'))));
    }

    #[test]
    fn test_star_operator() {
        let ast = parse("a*");
        assert_eq!(ast, Star(boxed(Byte(b'a'))));
    }

    #[test]
    fn test_star_operator_nested() {
        let ast_nested = parse("ab*");
        assert_eq!(
            ast_nested,
            Concat(boxed(Byte(b'a')), boxed(Star(boxed(Byte(b'b')))))
        );
    }

    #[test]
    fn test_alternation() {
        let ast = parse("a|b");
        assert_eq!(ast, Alt(boxed(Byte(b'a')), boxed(Byte(b'b'))));
    }

    #[test]
    fn test_concat_has_higher_precedence_than_alt() {
        let ast = parse("ab|c");
        assert_eq!(
            ast,
            Alt(
                boxed(Concat(boxed(Byte(b'a')), boxed(Byte(b'b')))),
                boxed(Byte(b'c'))
            )
        );
    }

    #[test]
    fn test_star_precedence() {
        let ast = parse("a*|b");
        assert_eq!(ast, Alt(boxed(Star(boxed(Byte(b'a')))), boxed(Byte(b'b'))));
    }

    #[test]
    fn test_grouping_affects_precedence() {
        let ast = parse("(a|b)c");
        assert_eq!(
            ast,
            Concat(
                boxed(Alt(boxed(Byte(b'a')), boxed(Byte(b'b')))),
                boxed(Byte(b'c'))
            )
        );
    }

    #[test]
    fn test_nested_groups() {
        let ast = parse("((a|b)*)c");
        assert_eq!(
            ast,
            Concat(
                boxed(Star(boxed(Alt(boxed(Byte(b'a')), boxed(Byte(b'b')))))),
                boxed(Byte(b'c'))
            )
        );
    }

    #[test]
    #[should_panic(expected = "Expected ')'")]
    fn test_unclosed_group_panics() {
        parse("(ab");
    }

    #[test]
    #[should_panic(expected = "Unexpected token")]
    fn test_invalid_start_token_panics() {
        parse("*a");
    }
}
