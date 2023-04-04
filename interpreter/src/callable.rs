use parser::utils::Visitor;
use std::fmt::{Debug, Display};

use crate::{environment::EnvironmentRef, result::IResult};

pub struct LoxCallable {
    pub arity: usize,
    pub call: Box<dyn FnMut(&mut dyn VisitorEnvironmentAware, Vec<IResult>) -> IResult>,
}

pub trait VisitorEnvironmentAware: Visitor<IResult> + EnvironmentAware {}

pub trait EnvironmentAware {
    fn get_environment(&self) -> EnvironmentRef;
    fn set_environment(&mut self, environment: EnvironmentRef);
}

impl Debug for LoxCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LoxCallable {{ arity: {}, call: fn }}", self.arity)
    }
}

impl Display for LoxCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq for LoxCallable {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
