use adam_regex::nfa::engine::{char_nfa, concat_nfa, matches, star_nfa};

#[test]
fn char_nfa_basic() {
    let nfa = char_nfa('a');
    assert!(matches(&nfa, "a"));
    assert!(!matches(&nfa, ""));
    assert!(!matches(&nfa, "b"));
    assert!(!matches(&nfa, "aa"));
}

#[test]
fn star_nfa_zero_or_more() {
    let nfa = star_nfa(char_nfa('a'));
    assert!(matches(&nfa, ""));
    assert!(matches(&nfa, "a"));
    assert!(matches(&nfa, "aa"));
    assert!(matches(&nfa, "aaaaaa"));
    assert!(!matches(&nfa, "b"));
    assert!(!matches(&nfa, "ab"));
}

#[test]
fn concat_two_chars() {
    let nfa = concat_nfa(&char_nfa('a'), &char_nfa('b'));
    assert!(matches(&nfa, "ab"));
    assert!(!matches(&nfa, ""));
    assert!(!matches(&nfa, "a"));
    assert!(!matches(&nfa, "b"));
    assert!(!matches(&nfa, "abc"));
}

#[test]
fn concat_with_star() {
    let a = char_nfa('a');
    let b_star = star_nfa(char_nfa('b'));
    let nfa = concat_nfa(&a, &b_star);

    assert!(matches(&nfa, "a"));
    assert!(matches(&nfa, "ab"));
    assert!(matches(&nfa, "abbb"));
    assert!(!matches(&nfa, "b"));
    assert!(!matches(&nfa, ""));
}

#[test]
fn star_of_concat() {
    let ab = concat_nfa(&char_nfa('a'), &char_nfa('b'));
    let star = star_nfa(ab);

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
    let base = char_nfa('x');
    let star1 = star_nfa(base);
    let star2 = star_nfa(star1);

    assert!(matches(&star2, ""));
    assert!(matches(&star2, "x"));
    assert!(matches(&star2, "xx"));
}

#[test]
fn invalid_inputs() {
    let nfa = concat_nfa(&char_nfa('a'), &char_nfa('b'));

    assert!(!matches(&nfa, "ba"));
    assert!(!matches(&nfa, "aa"));
    assert!(!matches(&nfa, "bb"));
    assert!(!matches(&nfa, "abc"));
}

#[test]
fn long_string_repetition() {
    let a_star = star_nfa(char_nfa('a'));
    let nfa = concat_nfa(&a_star, &char_nfa('b'));

    assert!(matches(&nfa, "b"));
    assert!(matches(&nfa, "ab"));
    assert!(matches(&nfa, "aaaaaaaab"));
    assert!(!matches(&nfa, "a"));
    assert!(!matches(&nfa, "aaaa"));
}

#[test]
fn empty_string_on_char_nfa() {
    let nfa = char_nfa('a');
    assert!(!matches(&nfa, ""));
}
