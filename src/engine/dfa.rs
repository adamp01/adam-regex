use bit_set::BitSet;

#[derive(Debug)]
pub struct DFA {
    pub states: Vec<[Option<usize>; 256]>,
    pub start: usize,
    pub accepting: BitSet,
}

impl DFA {
    pub fn matches(&self, input: &str) -> bool {
        let mut state = self.start;

        for &b in input.as_bytes() {
            match self.states[state][b as usize] {
                Some(next) => state = next,
                None => return false,
            }
        }

        self.accepting.contains(state)
    }
}
