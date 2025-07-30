use adam_regex::regex::lexer::Lexer;
use adam_regex::regex::parser::{Parser, Regex};

fn parse(input: &str) -> Regex {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse()
}

fn boxed(expr: Regex) -> Box<Regex> {
    Box::new(expr)
}

#[test]
fn test_single_char() {
    let ast = parse("a");
    assert_eq!(ast, Regex::Char('a'));
}

#[test]
fn test_concat_two_chars() {
    let ast = parse("ab");
    assert_eq!(
        ast,
        Regex::Concat(boxed(Regex::Char('a')), boxed(Regex::Char('b')))
    );
}

#[test]
fn test_star_operator() {
    let ast = parse("a*");
    assert_eq!(ast, Regex::Star(boxed(Regex::Char('a'))));
}

#[test]
fn test_star_operator_nested() {
    let ast_nested = parse("ab*");
    assert_eq!(
        ast_nested,
        Regex::Concat(
            boxed(Regex::Char('a')),
            boxed(Regex::Star(boxed(Regex::Char('b'))))
        )
    );
}

#[test]
fn test_alternation() {
    let ast = parse("a|b");
    assert_eq!(
        ast,
        Regex::Alt(boxed(Regex::Char('a')), boxed(Regex::Char('b')))
    );
}

#[test]
fn test_concat_has_higher_precedence_than_alt() {
    let ast = parse("ab|c");
    assert_eq!(
        ast,
        Regex::Alt(
            boxed(Regex::Concat(
                boxed(Regex::Char('a')),
                boxed(Regex::Char('b'))
            )),
            boxed(Regex::Char('c'))
        )
    );
}

#[test]
fn test_star_precedence() {
    let ast = parse("a*|b");
    assert_eq!(
        ast,
        Regex::Alt(
            boxed(Regex::Star(boxed(Regex::Char('a')))),
            boxed(Regex::Char('b'))
        )
    );
}

#[test]
fn test_grouping_affects_precedence() {
    let ast = parse("(a|b)c");
    assert_eq!(
        ast,
        Regex::Concat(
            boxed(Regex::Alt(boxed(Regex::Char('a')), boxed(Regex::Char('b')))),
            boxed(Regex::Char('c'))
        )
    );
}

#[test]
fn test_nested_groups() {
    let ast = parse("((a|b)*)c");
    assert_eq!(
        ast,
        Regex::Concat(
            boxed(Regex::Star(boxed(Regex::Alt(
                boxed(Regex::Char('a')),
                boxed(Regex::Char('b'))
            )))),
            boxed(Regex::Char('c'))
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
