use adam_regex::regex::lexer::{Lexer, Token};

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
    assert_eq!(lex_all("a"), vec![Token::Char('a'), Token::EOF]);
    assert_eq!(lex_all("*"), vec![Token::Star, Token::EOF]);
    assert_eq!(lex_all("|"), vec![Token::Alt, Token::EOF]);
    assert_eq!(lex_all("("), vec![Token::LParen, Token::EOF]);
    assert_eq!(lex_all(")"), vec![Token::RParen, Token::EOF]);
}

#[test]
fn multi_token_sequence() {
    assert_eq!(
        lex_all("a*b|(c)"),
        vec![
            Token::Char('a'),
            Token::Star,
            Token::Char('b'),
            Token::Alt,
            Token::LParen,
            Token::Char('c'),
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
#[should_panic(expected = "Unknown character: +")]
fn invalid_character_plus() {
    let mut lexer = Lexer::new("a+");
    lexer.next_token(); // 'a'
    lexer.next_token(); // should panic on '+'
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
            Token::Char('a'),
            Token::Char('z'),
            Token::Char('A'),
            Token::Char('Z'),
            Token::Char('0'),
            Token::Char('1'),
            Token::Char('9'),
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
            Token::Char('a'),
            Token::Char('a'),
            Token::Star,
            Token::RParen,
            Token::Alt,
            Token::LParen,
            Token::Char('b'),
            Token::Char('b'),
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
            Token::Char('9'),
            Token::EOF
        ]
    );
}
