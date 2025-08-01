use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Regex {
    Byte(u8),
    Star(Box<Regex>),
    Concat(Box<Regex>, Box<Regex>),
    Alt(Box<Regex>, Box<Regex>),
}

impl fmt::Display for Regex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn needs_parens(inner: &Regex, outer_prec: u8) -> bool {
            let inner_prec = match inner {
                Regex::Alt(_, _) => 1,
                Regex::Concat(_, _) => 2,
                Regex::Star(_) => 3,
                Regex::Byte(_) => 4,
            };
            inner_prec < outer_prec
        }

        fn write_expr(expr: &Regex, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match expr {
                Regex::Byte(b) => {
                    let c = *b as char;
                    if c.is_ascii_graphic() && !"()*|".contains(c) {
                        write!(f, "{}", c)
                    } else {
                        write!(f, "\\x{:02X}", b)
                    }
                }
                Regex::Star(inner) => {
                    if needs_parens(inner, 3) {
                        write!(f, "({})*", inner)
                    } else {
                        write!(f, "{}*", inner)
                    }
                }
                Regex::Concat(left, right) => {
                    if needs_parens(left, 2) {
                        write!(f, "({})", left)?;
                    } else {
                        write!(f, "{}", left)?;
                    }

                    if needs_parens(right, 2) {
                        write!(f, "({})", right)
                    } else {
                        write!(f, "{}", right)
                    }
                }
                Regex::Alt(left, right) => {
                    if needs_parens(left, 1) {
                        write!(f, "({})", left)?;
                    } else {
                        write!(f, "{}", left)?;
                    }

                    write!(f, "|")?;

                    if needs_parens(right, 1) {
                        write!(f, "({})", right)
                    } else {
                        write!(f, "{}", right)
                    }
                }
            }
        }

        write_expr(self, f)
    }
}
