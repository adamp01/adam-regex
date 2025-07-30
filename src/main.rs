use adam_regex::regex::parser::parse;
use adam_regex::nfa::engine::{from_regex, matches};

fn main() {
    let regex = parse(&"(a|b)*c");
    let nfa = from_regex(&regex);

    assert!(matches(&nfa, "aaaabbbbbc"))
}
