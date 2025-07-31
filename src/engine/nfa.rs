use std::collections::{BTreeSet, HashMap, VecDeque};

use crate::{ast::Regex, engine::dfa::DFA};

#[derive(Debug, Clone, PartialEq)]
pub enum Transition {
    Char(char),
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

    fn epsilon_closure(&self, states: &BTreeSet<usize>) -> BTreeSet<usize> {
        let mut closure = states.clone();
        let mut stack: Vec<usize> = states.iter().cloned().collect();

        while let Some(state) = stack.pop() {
            for edge in &self.states[state].edges {
                if let Transition::Epsilon = edge.label {
                    if !closure.contains(&edge.to) {
                        closure.insert(edge.to);
                        stack.push(edge.to);
                    }
                }
            }
        }
        closure
    }

    fn move_on(&self, states: &BTreeSet<usize>, input: char) -> BTreeSet<usize> {
        let mut result = BTreeSet::new();

        for &state in states {
            for edge in &self.states[state].edges {
                if let Transition::Char(c) = edge.label {
                    if c == input {
                        result.insert(edge.to);
                    }
                }
            }
        }
        result
    }

    fn extract_alphabet(&self) -> BTreeSet<char> {
        let mut chars = BTreeSet::new();
        for state in &self.states {
            for edge in &state.edges {
                if let Transition::Char(c) = edge.label {
                    chars.insert(c);
                }
            }
        }
        chars
    }

    pub fn to_dfa(&self) -> DFA {
        let alphabet = self.extract_alphabet();
        let mut state_map = HashMap::new();
        let mut dfa_states = vec![];
        let mut accepting = BTreeSet::new();

        let start_set = self.epsilon_closure(&BTreeSet::from([self.start]));
        state_map.insert(start_set.clone(), 0);
        dfa_states.push(HashMap::new());

        let mut queue = VecDeque::new();
        queue.push_back(start_set.clone());

        while let Some(current_set) = queue.pop_front() {
            let current_idx = state_map[&current_set];
            for &c in &alphabet {
                let move_set = self.move_on(&current_set, c);
                if move_set.is_empty() {
                    continue;
                }
                let next_set = self.epsilon_closure(&move_set);

                let next_idx = *state_map.entry(next_set.clone()).or_insert_with(|| {
                    let idx = dfa_states.len();
                    dfa_states.push(HashMap::new());
                    queue.push_back(next_set.clone());
                    idx
                });

                dfa_states[current_idx].insert(c, next_idx);
            }

            if current_set.contains(&self.accept) {
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
        Regex::Char(c) => {
            let mut nfa = NFA {
                states: vec![],
                start: 0,
                accept: 0,
            };
            let start = nfa.new_state();
            let end = nfa.new_state();
            nfa.start = start;
            nfa.accept = end;
            nfa.add_transition(start, end, Transition::Char(*c));
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
        let actual = from_regex(&Char('a'));

        let expected = NFA {
            states: vec![
                State {
                    edges: vec![Edge {
                        label: Transition::Char('a'),
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
        let actual = from_regex(&Concat(b(Char('x')), b(Char('y'))));

        let expected = NFA {
            states: vec![
                State {
                    edges: vec![Edge {
                        label: Transition::Char('x'),
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
                        label: Transition::Char('y'),
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
        let actual = from_regex(&Alt(b(Char('a')), b(Char('b'))));

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
                        label: Transition::Char('a'),
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
                        label: Transition::Char('b'),
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
        let actual = from_regex(&Star(Box::new(Char('z'))));

        let expected = NFA {
            states: vec![
                State {
                    edges: vec![Edge {
                        label: Transition::Char('z'),
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
                        label: Transition::Char('z'),
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
