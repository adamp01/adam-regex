use adam_regex::parser::{parse, Regex::{self, *}};

fn boxed(expr: Regex) -> Box<Regex> {
    Box::new(expr)
}

#[test]
fn test_single_char() {
    let ast = parse("a");
    assert_eq!(ast, Char('a'));
}

#[test]
fn test_concat_two_chars() {
    let ast = parse("ab");
    assert_eq!(
        ast,
        Concat(boxed(Char('a')), boxed(Char('b')))
    );
}

#[test]
fn test_star_operator() {
    let ast = parse("a*");
    assert_eq!(ast, Star(boxed(Char('a'))));
}

#[test]
fn test_star_operator_nested() {
    let ast_nested = parse("ab*");
    assert_eq!(
        ast_nested,
        Concat(
            boxed(Char('a')),
            boxed(Star(boxed(Char('b'))))
        )
    );
}

#[test]
fn test_alternation() {
    let ast = parse("a|b");
    assert_eq!(
        ast,
        Alt(boxed(Char('a')), boxed(Char('b')))
    );
}

#[test]
fn test_concat_has_higher_precedence_than_alt() {
    let ast = parse("ab|c");
    assert_eq!(
        ast,
        Alt(
            boxed(Concat(
                boxed(Char('a')),
                boxed(Char('b'))
            )),
            boxed(Char('c'))
        )
    );
}

#[test]
fn test_star_precedence() {
    let ast = parse("a*|b");
    assert_eq!(
        ast,
        Alt(
            boxed(Star(boxed(Char('a')))),
            boxed(Char('b'))
        )
    );
}

#[test]
fn test_grouping_affects_precedence() {
    let ast = parse("(a|b)c");
    assert_eq!(
        ast,
        Concat(
            boxed(Alt(boxed(Char('a')), boxed(Char('b')))),
            boxed(Char('c'))
        )
    );
}

#[test]
fn test_nested_groups() {
    let ast = parse("((a|b)*)c");
    assert_eq!(
        ast,
        Concat(
            boxed(Star(boxed(Alt(
                boxed(Char('a')),
                boxed(Char('b'))
            )))),
            boxed(Char('c'))
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
