use adam_regex::matcher::AdamRegex;

fn main() {
    let re = AdamRegex::from_str(&"(a|b)*c").unwrap();

    assert!(re.matches("aaaabbbbbc"))
}
