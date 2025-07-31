use adam_regex::matcher::{from_regex, matches};
use adam_regex::parser::Regex::{self, *};

fn b(r: Regex) -> Box<Regex> {
    Box::new(r)
}

#[test]
fn char_nfa_basic() {
    // 'a'
    let nfa = from_regex(&Char('a'));
    assert!(matches(&nfa, "a"));
    assert!(!matches(&nfa, ""));
    assert!(!matches(&nfa, "b"));
    assert!(!matches(&nfa, "aa"));
}

#[test]
fn star_nfa_zero_or_more() {
    // 'a*'
    let nfa = from_regex(&Star(b(Char('a'))));
    assert!(matches(&nfa, ""));
    assert!(matches(&nfa, "a"));
    assert!(matches(&nfa, "aa"));
    assert!(matches(&nfa, "aaaaaa"));
    assert!(!matches(&nfa, "b"));
    assert!(!matches(&nfa, "ab"));
}

#[test]
fn concat_two_chars() {
    // 'ab'
    let nfa = from_regex(&Concat(b(Char('a')), b(Char('b'))));
    assert!(matches(&nfa, "ab"));
    assert!(!matches(&nfa, ""));
    assert!(!matches(&nfa, "a"));
    assert!(!matches(&nfa, "b"));
    assert!(!matches(&nfa, "abc"));
}

#[test]
fn concat_with_star() {
    // 'ab*'
    let a = Char('a');
    let b_star = Star(b(Char('b')));
    let nfa = from_regex(&Concat(b(a), b(b_star)));

    assert!(matches(&nfa, "a"));
    assert!(matches(&nfa, "ab"));
    assert!(matches(&nfa, "abbb"));
    assert!(!matches(&nfa, "b"));
    assert!(!matches(&nfa, ""));
}

#[test]
fn star_of_concat() {
    // '(ab)*'
    let ab = Concat(b(Char('a')), b(Char('b')));
    let star = from_regex(&Star(b(ab)));

    assert!(matches(&star, ""));
    assert!(matches(&star, "ab"));
    assert!(matches(&star, "abab"));
    assert!(matches(&star, "ababab"));
    assert!(!matches(&star, "a"));
    assert!(!matches(&star, "b"));
    assert!(!matches(&star, "aba"));
}

#[test]
fn nested_star_star() {
    // 'x**'
    let base = Char('x');
    let star1 = Star(b(base));
    let star2 = from_regex(&Star(b(star1)));

    assert!(matches(&star2, ""));
    assert!(matches(&star2, "x"));
    assert!(matches(&star2, "xx"));
}

#[test]
fn alt_star() {
    // '(a|b)*'
    let alt = Star(b(Alt(b(Char('a')), b(Char('b')))));
    let nfa = from_regex(&alt);

    assert!(matches(&nfa, "aaa"));
    assert!(matches(&nfa, "bbb"));
    assert!(matches(&nfa, "abbb"));
    assert!(matches(&nfa, "aaab"));
    assert!(!matches(&nfa, "abc"));
}

#[test]
fn long_string_repetition() {
    // 'a*b'
    let a_star = Star(b(Char('a')));
    let nfa = from_regex(&Concat(b(a_star), b(Char('b'))));

    assert!(matches(&nfa, "b"));
    assert!(matches(&nfa, "ab"));
    assert!(matches(&nfa, "aaaaaaaab"));
    assert!(!matches(&nfa, "a"));
    assert!(!matches(&nfa, "aaaa"));
}

#[test]
fn empty_string_on_char_nfa() {
    let nfa = from_regex(&Char('a'));
    assert!(!matches(&nfa, ""));
}
