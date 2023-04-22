use std::{fmt::Display, rc::Rc};

use rustcore::Shared;

use crate::callable::LoxCallable;

#[derive(Debug, PartialEq)]
pub enum IResult {
    Number(f64),
    String(Rc<String>),
    Bool(bool),
    None,
    Break,
    Callable(Shared<LoxCallable>),
    Return(Rc<IResult>),
}

impl Clone for IResult {
    fn clone(&self) -> Self {
        match self {
            Self::Number(arg0) => Self::Number(*arg0),
            Self::String(arg0) => Self::String(arg0.clone()),
            Self::Bool(arg0) => Self::Bool(*arg0),
            Self::None => Self::None,
            Self::Break => Self::Break,
            Self::Callable(arg0) => Self::Callable(arg0.clone()),
            Self::Return(arg0) => Self::Return(arg0.clone()),
        }
    }
}

impl Display for IResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::None => write!(f, ""),
            Self::Break => write!(f, "break"),
            Self::Callable(c) => write!(f, "{}", c),
            Self::Return(r) => write!(f, "<return>{}", r),
        }
    }
}
