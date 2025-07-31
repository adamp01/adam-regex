use adam_regex::matcher::Pattern;

fn main() {
    let pattern = Pattern::from_str(&"(a|b)*c").unwrap();

    assert!(pattern.matches("aaaabbbbbc"))
}
