use adam_regex::nfa::{from_regex};
use adam_regex::parser::Regex::{self, *};

fn b(r: Regex) -> Box<Regex> {
    Box::new(r)
}

#[test]
fn char_nfa_basic() {
    // 'a'
    let nfa = from_regex(&Char('a'));
    assert!(nfa.matches("a"));
    assert!(!nfa.matches(""));
    assert!(!nfa.matches("b"));
    assert!(!nfa.matches("aa"));
}

#[test]
fn star_nfa_zero_or_more() {
    // 'a*'
    let nfa = from_regex(&Star(b(Char('a'))));
    assert!(nfa.matches(""));
    assert!(nfa.matches("a"));
    assert!(nfa.matches("aa"));
    assert!(nfa.matches("aaaaaa"));
    assert!(!nfa.matches("b"));
    assert!(!nfa.matches("ab"));
}

#[test]
fn concat_two_chars() {
    // 'ab'
    let nfa = from_regex(&Concat(b(Char('a')), b(Char('b'))));
    assert!(nfa.matches("ab"));
    assert!(!nfa.matches(""));
    assert!(!nfa.matches("a"));
    assert!(!nfa.matches("b"));
    assert!(!nfa.matches("abc"));
}

#[test]
fn concat_with_star() {
    // 'ab*'
    let a = Char('a');
    let b_star = Star(b(Char('b')));
    let nfa = from_regex(&Concat(b(a), b(b_star)));

    assert!(nfa.matches("a"));
    assert!(nfa.matches("ab"));
    assert!(nfa.matches("abbb"));
    assert!(!nfa.matches("b"));
    assert!(!nfa.matches(""));
}

#[test]
fn star_of_concat() {
    // '(ab)*'
    let ab = Concat(b(Char('a')), b(Char('b')));
    let nfa= from_regex(&Star(b(ab)));

    assert!(nfa.matches(""));
    assert!(nfa.matches("ab"));
    assert!(nfa.matches("abab"));
    assert!(nfa.matches("ababab"));
    assert!(!nfa.matches("a"));
    assert!(!nfa.matches("b"));
    assert!(!nfa.matches("aba"));
}

#[test]
fn nested_star_star() {
    // 'x**'
    let base = Char('x');
    let star1 = Star(b(base));
    let nfa= from_regex(&Star(b(star1)));

    assert!(nfa.matches(""));
    assert!(nfa.matches("x"));
    assert!(nfa.matches("xx"));
}

#[test]
fn alt_star() {
    // '(a|b)*'
    let alt = Star(b(Alt(b(Char('a')), b(Char('b')))));
    let nfa = from_regex(&alt);

    assert!(nfa.matches("aaa"));
    assert!(nfa.matches("bbb"));
    assert!(nfa.matches("abbb"));
    assert!(nfa.matches("aaab"));
    assert!(!nfa.matches("abc"));
}

#[test]
fn long_string_repetition() {
    // 'a*b'
    let a_star = Star(b(Char('a')));
    let nfa = from_regex(&Concat(b(a_star), b(Char('b'))));

    assert!(nfa.matches("b"));
    assert!(nfa.matches("ab"));
    assert!(nfa.matches("aaaaaaaab"));
    assert!(!nfa.matches("a"));
    assert!(!nfa.matches("aaaa"));
}

#[test]
fn empty_string_on_char_nfa() {
    let nfa = from_regex(&Char('a'));
    assert!(!nfa.matches(""));
}
