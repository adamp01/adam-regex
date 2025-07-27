mod nfa;
use crate::nfa::engine::{char_nfa, concat_nfa, matches, star_nfa};

fn main() {
    let nfa_a = star_nfa(char_nfa('a'));
    let nfa_b = star_nfa(char_nfa('b'));
    let final_nfa = concat_nfa(&concat_nfa(&nfa_a, &nfa_b), &nfa_a);

    assert_eq!(matches(&final_nfa, "abbb"), true);
    assert_eq!(matches(&final_nfa, "a"), true);
    assert_eq!(matches(&final_nfa, "ab"), true);
    assert_eq!(matches(&final_nfa, "ac"), false);
    assert_eq!(matches(&final_nfa, "aaabbb"), true);
    assert_eq!(matches(&final_nfa, "aaabbbaaa"), true);
}
