#[derive(Debug, Clone)]
pub enum Regex {
    Char(char),
    Star(Box<Regex>),
    Concat(Box<Regex>, Box<Regex>),
    Alt(Box<Regex>, Box<Regex>),
}
