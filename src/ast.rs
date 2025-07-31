#[derive(Debug, Clone, PartialEq)]
pub enum Regex {
    Byte(u8),
    Star(Box<Regex>),
    Concat(Box<Regex>, Box<Regex>),
    Alt(Box<Regex>, Box<Regex>),
}
