use crate::{
    ast::Regex,
    engine::{compiler, dfa::DFA},
    parser::parser,
};

pub struct AdamRegex {
    dfa: DFA,
}

impl AdamRegex {
    pub fn from_str(input: &str) -> Result<Self, String> {
        let ast = parser::parse(input);
        let dfa = compiler::compile(&ast);
        Ok(Self { dfa })
    }

    pub fn from_ast(ast: &Regex) -> Self {
        let dfa = compiler::compile(ast);
        Self { dfa }
    }

    pub fn matches(&self, input: &str) -> bool {
        self.dfa.matches(input)
    }
}
