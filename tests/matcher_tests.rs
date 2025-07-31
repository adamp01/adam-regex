use adam_regex::ast::Regex::{self, *};
use adam_regex::matcher::AdamRegex;

fn b(r: Regex) -> Box<Regex> {
    Box::new(r)
}

fn dfa_from(r: &Regex) -> AdamRegex {
    AdamRegex::from_ast(r)
}

#[test]
fn char_basic() {
    let dfa = dfa_from(&Byte(b'a'));
    assert!(dfa.matches("a"));
    assert!(!dfa.matches(""));
    assert!(!dfa.matches("b"));
    assert!(!dfa.matches("aa"));
}

#[test]
fn star() {
    let dfa = dfa_from(&Star(b(Byte(b'a'))));
    assert!(dfa.matches(""));
    assert!(dfa.matches("a"));
    assert!(dfa.matches("aaaa"));
    assert!(!dfa.matches("ab"));
}

#[test]
fn concat() {
    let dfa = dfa_from(&Concat(b(Byte(b'a')), b(Byte(b'b'))));
    assert!(dfa.matches("ab"));
    assert!(!dfa.matches("a"));
    assert!(!dfa.matches(""));
    assert!(!dfa.matches("abc"));
}

#[test]
fn star_of_concat() {
    let ab = Concat(b(Byte(b'a')), b(Byte(b'b')));
    let dfa = dfa_from(&Star(b(ab)));
    assert!(dfa.matches(""));
    assert!(dfa.matches("ab"));
    assert!(dfa.matches("abab"));
    assert!(!dfa.matches("a"));
    assert!(!dfa.matches("aba"));
}

#[test]
fn nested_star_star() {
    let inner = Star(b(Byte(b'x')));
    let outer = Star(b(inner));
    let dfa = dfa_from(&outer);
    assert!(dfa.matches(""));
    assert!(dfa.matches("x"));
    assert!(dfa.matches("xx"));
}

#[test]
fn alt_star() {
    let alt = Alt(b(Byte(b'a')), b(Byte(b'b')));
    let dfa = dfa_from(&Star(b(alt)));
    assert!(dfa.matches(""));
    assert!(dfa.matches("a"));
    assert!(dfa.matches("abab"));
    assert!(!dfa.matches("abc"));
}

#[test]
fn long_repetition() {
    let pattern = Concat(b(Star(b(Byte(b'a')))), b(Byte(b'b')));
    let dfa = dfa_from(&pattern);
    assert!(dfa.matches("b"));
    assert!(dfa.matches("ab"));
    assert!(dfa.matches("aaaaab"));
    assert!(!dfa.matches("a"));
    assert!(!dfa.matches(""));
}
