use crate::ast::Regex;
use crate::engine::dfa::DFA;
use crate::engine::nfa::from_regex;

pub fn compile(ast: &Regex) -> DFA {
    let nfa = from_regex(&ast);
    nfa.to_dfa()
}
