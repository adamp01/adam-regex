use std::collections::{HashMap, VecDeque};

use bit_set::BitSet;

use crate::{ast::Regex, engine::dfa::DFA};

#[derive(Debug, Clone, PartialEq)]
pub enum Transition {
    Byte(u8),
    Epsilon,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub label: Transition,
    pub to: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub edges: Vec<Edge>,
}

#[derive(Debug, PartialEq)]
pub struct NFA {
    pub states: Vec<State>,
    pub start: usize,
    pub accept: usize,
}

impl NFA {
    fn new_state(&mut self) -> usize {
        self.states.push(State { edges: vec![] });
        self.states.len() - 1
    }

    fn add_transition(&mut self, from: usize, to: usize, label: Transition) {
        self.states[from].edges.push(Edge { label, to })
    }

    fn offset(&mut self, offset: usize) {
        self.states.iter_mut().for_each(|s| {
            s.edges.iter_mut().for_each(|e| e.to += offset);
        });
        self.start += offset;
        self.accept += offset;
    }

    fn epsilon_closure(&self, states: &BitSet) -> BitSet {
        let mut closure = BitSet::with_capacity(self.states.len());
        let mut stack: Vec<usize> = Vec::new();

        for state in states.iter() {
            if closure.insert(state) {
                stack.push(state);
            }
        }

        while let Some(state) = stack.pop() {
            for edge in &self.states[state].edges {
                if let Transition::Epsilon = edge.label {
                    if closure.insert(edge.to) {
                        stack.push(edge.to);
                    }
                }
            }
        }
        closure
    }

    fn move_on(&self, states: &BitSet, byte: u8) -> BitSet {
        let mut next = BitSet::with_capacity(self.states.len());

        for state in states {
            for edge in &self.states[state].edges {
                if let Transition::Byte(b) = edge.label {
                    if b == byte {
                        next.insert(edge.to);
                    }
                }
            }
        }
        next 
    }

    pub fn to_dfa(&self) -> DFA {
        let mut state_map = HashMap::new();
        let mut dfa_states = Vec::new();
        let mut accepting = BitSet::with_capacity(self.states.len());

        let mut queue = VecDeque::new();

        let mut start_set = BitSet::with_capacity(self.states.len());
        start_set.insert(self.start);
        let start_closure = self.epsilon_closure(&start_set);

        state_map.insert(start_closure.clone(), 0);
        dfa_states.push([None; 256]);

        queue.push_back(start_closure.clone());

        while let Some(current_set) = queue.pop_front() {
            let current_idx = state_map[&current_set];

            for byte in 0u8..=255 {
                let move_set = self.move_on(&current_set, byte);
                if move_set.is_empty() {
                    continue;
                }
                let next_set = self.epsilon_closure(&move_set);

                let next_idx = *state_map.entry(next_set.clone()).or_insert_with(|| {
                    let idx = dfa_states.len();
                    dfa_states.push([None; 256]);
                    queue.push_back(next_set.clone());
                    idx
                });

                dfa_states[current_idx][byte as usize] = Some(next_idx);
            }

            if current_set.contains(self.accept) {
                accepting.insert(current_idx);
            }
        }

        DFA {
            states: dfa_states,
            start: 0,
            accepting,
        }
    }
}

pub fn from_regex(regex: &Regex) -> NFA {
    match regex {
        Regex::Byte(b) => {
            let mut nfa = NFA {
                states: vec![],
                start: 0,
                accept: 0,
            };
            let start = nfa.new_state();
            let end = nfa.new_state();
            nfa.start = start;
            nfa.accept = end;
            nfa.add_transition(start, end, Transition::Byte(*b));
            nfa
        }

        Regex::Concat(left, right) => {
            let a = from_regex(left);
            let mut b = from_regex(right);

            let offset = a.states.len();
            b.offset(offset);

            let mut states = a.states;
            states[a.accept].edges.push(Edge {
                label: Transition::Epsilon,
                to: b.start,
            });
            states.extend(b.states);

            NFA {
                states,
                start: a.start,
                accept: b.accept,
            }
        }

        Regex::Alt(left, right) => {
            let mut a = from_regex(left);
            let mut b = from_regex(right);

            let mut nfa = NFA {
                states: vec![],
                start: 0,
                accept: 0,
            };
            let start = nfa.new_state();

            let offset_a = nfa.states.len();
            a.offset(offset_a);
            nfa.states.extend(a.states);

            let offset_b = nfa.states.len();
            b.offset(offset_b);
            nfa.states.extend(b.states);

            let accept = nfa.new_state();

            nfa.add_transition(start, a.start, Transition::Epsilon);
            nfa.add_transition(start, b.start, Transition::Epsilon);
            nfa.add_transition(a.accept, accept, Transition::Epsilon);
            nfa.add_transition(b.accept, accept, Transition::Epsilon);

            nfa.start = start;
            nfa.accept = accept;
            nfa
        }

        Regex::Star(inner) => {
            let base = from_regex(inner);
            let mut nfa = NFA {
                states: base.states.clone(),
                start: 0,
                accept: 0,
            };
            let start = nfa.new_state();
            let accept = nfa.new_state();

            nfa.add_transition(start, base.start, Transition::Epsilon);
            nfa.add_transition(start, accept, Transition::Epsilon);
            nfa.add_transition(base.accept, base.start, Transition::Epsilon);
            nfa.add_transition(base.accept, accept, Transition::Epsilon);

            nfa.states.extend(base.states);
            nfa.start = start;
            nfa.accept = accept;
            nfa
        }
    }
}

#[cfg(test)]
mod structure_tests {
    use super::*;
    use crate::ast::Regex::{self, *};

    fn b(r: Regex) -> Box<Regex> {
        Box::new(r)
    }

    #[test]
    fn from_regex_char_structure() {
        let actual = from_regex(&Byte(b'a'));

        let expected = NFA {
            states: vec![
                State {
                    edges: vec![Edge {
                        label: Transition::Byte(b'a'),
                        to: 1,
                    }],
                },
                State { edges: vec![] },
            ],
            start: 0,
            accept: 1,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn from_regex_concat_structure() {
        let actual = from_regex(&Concat(b(Byte(b'x')), b(Byte(b'y'))));

        let expected = NFA {
            states: vec![
                State {
                    edges: vec![Edge {
                        label: Transition::Byte(b'x'),
                        to: 1,
                    }],
                },
                State {
                    edges: vec![Edge {
                        label: Transition::Epsilon,
                        to: 2,
                    }],
                },
                State {
                    edges: vec![Edge {
                        label: Transition::Byte(b'y'),
                        to: 3,
                    }],
                },
                State { edges: vec![] },
            ],
            start: 0,
            accept: 3,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn from_regex_alt_structure() {
        let actual = from_regex(&Alt(b(Byte(b'a')), b(Byte(b'b'))));

        let expected = NFA {
            states: vec![
                // state 0: start
                State {
                    edges: vec![
                        Edge {
                            label: Transition::Epsilon,
                            to: 1,
                        },
                        Edge {
                            label: Transition::Epsilon,
                            to: 3,
                        },
                    ],
                },
                // state 1: a start
                State {
                    edges: vec![Edge {
                        label: Transition::Byte(b'a'),
                        to: 2,
                    }],
                },
                // state 2: a accept
                State {
                    edges: vec![Edge {
                        label: Transition::Epsilon,
                        to: 5,
                    }],
                },
                // state 3: b start
                State {
                    edges: vec![Edge {
                        label: Transition::Byte(b'b'),
                        to: 4,
                    }],
                },
                // state 4: b accept
                State {
                    edges: vec![Edge {
                        label: Transition::Epsilon,
                        to: 5,
                    }],
                },
                // state 5: final accept
                State { edges: vec![] },
            ],
            start: 0,
            accept: 5,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn from_regex_star_structure() {
        let actual = from_regex(&Star(Box::new(Byte(b'z'))));

        let expected = NFA {
            states: vec![
                State {
                    edges: vec![Edge {
                        label: Transition::Byte(b'z'),
                        to: 1,
                    }],
                },
                State {
                    edges: vec![
                        Edge {
                            label: Transition::Epsilon,
                            to: 0,
                        },
                        Edge {
                            label: Transition::Epsilon,
                            to: 3,
                        },
                    ],
                },
                State {
                    edges: vec![
                        Edge {
                            label: Transition::Epsilon,
                            to: 0,
                        },
                        Edge {
                            label: Transition::Epsilon,
                            to: 3,
                        },
                    ],
                },
                State { edges: vec![] },
                State {
                    edges: vec![Edge {
                        label: Transition::Byte(b'z'),
                        to: 1,
                    }],
                },
                State { edges: vec![] },
            ],
            start: 2,
            accept: 3,
        };

        assert_eq!(actual, expected);
    }
}
