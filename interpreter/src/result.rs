use std::{fmt::Display, rc::Rc};

#[derive(Debug, PartialEq)]
pub enum IResult {
    Number(f64),
    String(Rc<String>),
    Bool(bool),
    None,
}

impl Clone for IResult {
    fn clone(&self) -> Self {
        match self {
            Self::Number(arg0) => Self::Number(arg0.clone()),
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Bool(arg0) => Self::Bool(arg0.clone()),
            Self::None => Self::None,
        }
    }
}

impl Display for IResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IResult::Number(n) => write!(f, "{}", n),
            IResult::String(s) => write!(f, "{}", s),
            IResult::Bool(b) => write!(f, "{}", b),
            IResult::None => write!(f, ""),
        }
    }
}
