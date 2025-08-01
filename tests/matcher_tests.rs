use adam_regex::ast::Regex::{self, *};
use adam_regex::engine::compiler;
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

#[test]
fn dot_matches_anything() {
    let pattern = Dot;
    let dfa = dfa_from(&pattern);
    assert!(dfa.matches("a"));
    assert!(dfa.matches("z"));
    assert!(dfa.matches("1"));
    assert!(!dfa.matches(""));
    assert!(!dfa.matches("ab"));
}

#[test]
fn plus_matches_one_or_more() {
    let pattern = Plus(b(Byte(b'a')));
    let dfa = dfa_from(&pattern);
    assert!(dfa.matches("a"));
    assert!(dfa.matches("aaaa"));
    assert!(!dfa.matches(""));
    assert!(!dfa.matches("b"));
}

#[test]
fn optional_matches_zero_or_one() {
    let pattern = Optional(b(Byte(b'a')));
    let dfa = dfa_from(&pattern);
    assert!(dfa.matches(""));
    assert!(dfa.matches("a"));
    assert!(!dfa.matches("aa"));
    assert!(!dfa.matches("b"));
}

#[test]
fn test_dfa_minimization_reduces_states() {
    let cases: Vec<Regex> = vec![
        // a*
        Star(b(Byte(b'a'))),
        // (a|b)*
        Star(b(Alt(b(Byte(b'a')), b(Byte(b'b'))))),
        // (a|b|a)
        Alt(b(Alt(b(Byte(b'a')), b(Byte(b'b')))), b(Byte(b'a'))),
        // ((a|b)*)*
        Star(b(Star(b(Alt(b(Byte(b'a')), b(Byte(b'b'))))))),
    ];

    for pattern in cases {
        let minimized = compiler::compile(&pattern, true);
        let original = compiler::compile(&pattern, false);

        assert!(
            minimized.states.len() < original.states.len(),
            "Expected minimized DFA to shrink for pattern '{}': {} → {}",
            pattern,
            original.states.len(),
            minimized.states.len()
        );
    }
}

#[test]
fn test_dfa_minimization_preserves_state_count() {
    let cases: Vec<Regex> = vec![
        // (a|a)
        Alt(b(Byte(b'a')), b(Byte(b'a'))),
        // (ab|ab)
        Alt(
            b(Concat(b(Byte(b'a')), b(Byte(b'b')))),
            b(Concat(b(Byte(b'a')), b(Byte(b'b')))),
        ),
    ];

    for pattern in cases {
        let minimized = compiler::compile(&pattern, true);
        let original = compiler::compile(&pattern, false);

        assert_eq!(
            minimized.states.len(),
            original.states.len(),
            "Expected no change in DFA size for pattern '{}'",
            pattern
        );
    }
}
