use std::{any::Any, rc::Rc};

use crate::tokens::Token;

pub enum ElementType {
    Literal,
    Grouping,
    Unary,
    Binary,
}

pub trait ExprT {
    fn element_type(&self) -> ElementType;
    fn as_any(&self) -> &dyn Any;
}

pub trait Visitor<Ret> {
    fn visit_expression(&self, expr: &Expression) -> Ret;
    fn visit_literal(&self, lit: &Literal) -> Ret;
    fn visit_grouping(&self, grp: &Grouping) -> Ret;
    fn visit_unary(&self, unr: &Unary) -> Ret;
    fn visit_binary(&self, bin: &Binary) -> Ret;
}

// expression

pub struct Expression {
    pub value: Rc<dyn ExprT>,
}

// literals
pub struct Literal {
    pub value: Token,
}

impl ExprT for Literal {
    fn element_type(&self) -> ElementType {
        ElementType::Literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// grouping
pub struct Grouping {
    pub expr: Expression,
}

impl ExprT for Grouping {
    fn element_type(&self) -> ElementType {
        ElementType::Grouping
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// unary
pub struct Unary {
    pub operator: Token,
    pub expr: Expression,
}

impl ExprT for Unary {
    fn element_type(&self) -> ElementType {
        ElementType::Unary
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// binary
pub struct Binary {
    pub left: Expression,
    pub operator: Token,
    pub right: Expression,
}

impl ExprT for Binary {
    fn element_type(&self) -> ElementType {
        ElementType::Binary
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
