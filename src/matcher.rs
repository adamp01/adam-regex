use std::collections::HashSet;

use crate::parser::Regex;

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

fn epsilon_closure(nfa: &NFA, states: &HashSet<usize>) -> HashSet<usize> {
    let mut closure = states.clone();
    let mut stack: Vec<usize> = states.iter().cloned().collect();

    while let Some(state) = stack.pop() {
        for edge in &nfa.states[state].edges {
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

pub fn matches(nfa: &NFA, input: &str) -> bool {
    let mut current_states: HashSet<usize> = epsilon_closure(nfa, &HashSet::from([nfa.start]));

    for c in input.chars() {
        let mut next_states = HashSet::new();

        for &state in &current_states {
            for edge in &nfa.states[state].edges {
                if let Transition::Char(ec) = edge.label {
                    if ec == c {
                        next_states.insert(edge.to);
                    }
                }
            }
        }

        current_states = epsilon_closure(nfa, &next_states);
    }

    current_states.contains(&nfa.accept)
}

#[cfg(test)]
mod structure_tests {
    use super::*;
    use crate::parser::Regex::{self, *};

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
