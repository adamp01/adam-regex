use adam_regex::parser::parse;
use adam_regex::nfa::{from_regex};

fn main() {
    let regex = parse(&"(a|b)*c");
    let nfa = from_regex(&regex);

    assert!(nfa.matches("aaaabbbbbc"))
}
