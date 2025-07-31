use adam_regex::parser::parse;
use adam_regex::matcher::{from_regex, matches};

fn main() {
    let regex = parse(&"(a|b)*c");
    let nfa = from_regex(&regex);

    assert!(matches(&nfa, "aaaabbbbbc"))
}
