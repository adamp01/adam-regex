use crate::ast::Regex;
use crate::engine::dfa::DFA;
use crate::engine::nfa::from_regex;

pub fn compile(ast: &Regex, minimize: bool) -> DFA {
    let nfa = from_regex(&ast);
    let dfa = nfa.to_dfa();
    if minimize {
        return dfa.minimize();
    }
    dfa
}
