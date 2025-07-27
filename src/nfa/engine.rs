use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum Transition {
    Char(char),
    Epsilon,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub label: Transition,
    pub to: usize,
}

#[derive(Debug, Clone)]
pub struct State {
    pub edges: Vec<Edge>,
}

#[derive(Debug)]
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
}

pub fn char_nfa(c: char) -> NFA {
    let mut nfa = NFA {
        states: vec![],
        start: 0,
        accept: 0,
    };
    let start = nfa.new_state();
    let accept = nfa.new_state();
    nfa.states[start].edges.push(Edge {
        label: Transition::Char(c),
        to: accept,
    });
    nfa.start = start;
    nfa.accept = accept;
    nfa
}

pub fn star_nfa(inner: NFA) -> NFA {
    let mut nfa = NFA {
        states: inner.states.clone(),
        start: 0,
        accept: 0,
    };
    let start = nfa.new_state();
    let accept = nfa.new_state();

    // ε-transition from start to inner start and to accept
    nfa.states[start].edges.push(Edge {
        label: Transition::Epsilon,
        to: inner.start,
    });
    nfa.states[start].edges.push(Edge {
        label: Transition::Epsilon,
        to: accept,
    });

    // ε-transition from inner accept to inner start and to accept
    nfa.states[inner.accept].edges.push(Edge {
        label: Transition::Epsilon,
        to: inner.start,
    });
    nfa.states[inner.accept].edges.push(Edge {
        label: Transition::Epsilon,
        to: accept,
    });

    nfa.start = start;
    nfa.accept = accept;
    nfa
}

pub fn concat_nfa(first: &NFA, second: &NFA) -> NFA {
    let offset = first.states.len();

    // Shift states in second NFA by offest
    let new_second_states: Vec<State> = second
        .states
        .clone()
        .into_iter()
        .map(|mut state| {
            state.edges = state
                .edges
                .into_iter()
                .map(|mut edge| {
                    edge.to += offset;
                    edge
                })
                .collect();
            state
        })
        .collect();

    // Add ε-transition from first's accept to second's offset start
    let mut nfa = NFA {
        states: [first.states.clone(), new_second_states].concat(),
        start: first.start,
        accept: second.accept + offset,
    };

    nfa.states[first.accept].edges.push(Edge {
        label: Transition::Epsilon,
        to: second.start + offset,
    });

    nfa
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
