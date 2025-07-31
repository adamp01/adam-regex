use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
pub struct DFA {
    pub states: Vec<HashMap<char, usize>>,
    pub start: usize,
    pub accepting: BTreeSet<usize>,
}

impl DFA {
    pub fn matches(&self, input: &str) -> bool {
        let mut state = self.start;

        for c in input.chars() {
            match self.states[state].get(&c) {
                Some(&next) => state = next,
                None => return false,
            }
        }

        self.accepting.contains(&state)
    }
}
