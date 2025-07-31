use adam_regex::ast::Regex::{self, *};
use adam_regex::matcher::Pattern;

fn b(r: Regex) -> Box<Regex> {
    Box::new(r)
}

fn dfa_from(r: &Regex) -> Pattern {
    Pattern::from_ast(r)
}

#[test]
fn char_basic() {
    let dfa = dfa_from(&Char('a'));
    assert!(dfa.matches("a"));
    assert!(!dfa.matches(""));
    assert!(!dfa.matches("b"));
    assert!(!dfa.matches("aa"));
}

#[test]
fn star() {
    let dfa = dfa_from(&Star(b(Char('a'))));
    assert!(dfa.matches(""));
    assert!(dfa.matches("a"));
    assert!(dfa.matches("aaaa"));
    assert!(!dfa.matches("ab"));
}

#[test]
fn concat() {
    let dfa = dfa_from(&Concat(b(Char('a')), b(Char('b'))));
    assert!(dfa.matches("ab"));
    assert!(!dfa.matches("a"));
    assert!(!dfa.matches(""));
    assert!(!dfa.matches("abc"));
}

#[test]
fn star_of_concat() {
    let ab = Concat(b(Char('a')), b(Char('b')));
    let dfa = dfa_from(&Star(b(ab)));
    assert!(dfa.matches(""));
    assert!(dfa.matches("ab"));
    assert!(dfa.matches("abab"));
    assert!(!dfa.matches("a"));
    assert!(!dfa.matches("aba"));
}

#[test]
fn nested_star_star() {
    let inner = Star(b(Char('x')));
    let outer = Star(b(inner));
    let dfa = dfa_from(&outer);
    assert!(dfa.matches(""));
    assert!(dfa.matches("x"));
    assert!(dfa.matches("xx"));
}

#[test]
fn alt_star() {
    let alt = Alt(b(Char('a')), b(Char('b')));
    let dfa = dfa_from(&Star(b(alt)));
    assert!(dfa.matches(""));
    assert!(dfa.matches("a"));
    assert!(dfa.matches("abab"));
    assert!(!dfa.matches("abc"));
}

#[test]
fn long_repetition() {
    let pattern = Concat(b(Star(b(Char('a')))), b(Char('b')));
    let dfa = dfa_from(&pattern);
    assert!(dfa.matches("b"));
    assert!(dfa.matches("ab"));
    assert!(dfa.matches("aaaaab"));
    assert!(!dfa.matches("a"));
    assert!(!dfa.matches(""));
}
