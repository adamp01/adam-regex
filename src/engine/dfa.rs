use std::collections::{HashMap, VecDeque};

use bit_set::BitSet;

#[derive(Debug)]
pub struct DFA {
    pub states: Vec<[Option<usize>; 256]>,
    pub start: usize,
    pub accepting: BitSet,
}

impl DFA {
    fn refine(&self, partition: &mut Vec<BitSet>, state_to_group: &mut Vec<usize>, num_states: usize) {
        let alphabet: Vec<u8> = (0u8..=255).collect();

        let mut worklist: VecDeque<(usize, u8)> = VecDeque::new();
        for (i, _) in partition.iter().enumerate() {
            for &c in &alphabet {
                worklist.push_back((i, c));
            }
        }

        while let Some((group_idx, c)) = worklist.pop_front() {
            let mut involved: HashMap<usize, BitSet> = HashMap::new();

            for state in 0..num_states {
                if let Some(&Some(target)) = self.states[state].get(c as usize) {
                    if partition[group_idx].contains(target) {
                        let g = state_to_group[state];
                        involved
                            .entry(g)
                            .or_insert_with(|| BitSet::with_capacity(num_states))
                            .insert(state);
                    }
                }
            }

            for (g, ref states_involved) in involved {
                let group = &partition[g];
                if states_involved.len() < group.len() {
                    let mut new_group = BitSet::with_capacity(num_states);
                    for state in states_involved.iter() {
                        new_group.insert(state);
                    }

                    let mut old_group = BitSet::with_capacity(num_states);
                    for state in group.iter() {
                        if !new_group.contains(state) {
                            old_group.insert(state);
                        }
                    }

                    partition[g] = new_group;
                    partition.push(old_group);
                    let new_group_idx = partition.len() - 1;

                    for state in partition[new_group_idx].iter() {
                        state_to_group[state] = new_group_idx;
                    }

                    for &a in &alphabet {
                        worklist.push_back((g, a));
                        worklist.push_back((new_group_idx, a));
                    }
                }
            }
        }
    }

    pub fn minimize(&self) -> DFA {
        let num_states = self.states.len();

        let mut partition: Vec<BitSet> = Vec::new();
        let mut state_to_group = vec![0; num_states];

        // Step 1: Initialize partition with accepting and non-accepting states
        let accepting = self.accepting.clone();
        let mut non_accepting = BitSet::with_capacity(num_states);
        for state in 0..num_states {
            if !accepting.contains(state) {
                non_accepting.insert(state);
            }
        }

        if !accepting.is_empty() {
            partition.push(accepting);
        }
        if !non_accepting.is_empty() {
            partition.push(non_accepting);
        }

        for (i, group) in partition.iter().enumerate() {
            for state in group.iter() {
                state_to_group[state] = i;
            }
        }

        // Step 2: Refinement loop
        self.refine(&mut partition, &mut state_to_group, num_states);

        // Step 3: Build new DFA
        let mut new_states = vec![[None; 256]; partition.len()];
        let mut new_accepting = BitSet::with_capacity(partition.len());
        let new_start = state_to_group[self.start];

        for (i, group) in partition.iter().enumerate() {
            let rep = group.iter().next().unwrap();
            for (b, &target) in self.states[rep].iter().enumerate() {
                if let Some(t) = target {
                    new_states[i][b] = Some(state_to_group[t]);
                }
            }

            if self.accepting.contains(rep) {
                new_accepting.insert(i);
            }
        }

        DFA {
            states: new_states,
            start: new_start,
            accepting: new_accepting,
        }
    }

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
