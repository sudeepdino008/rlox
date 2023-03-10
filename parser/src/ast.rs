use as_any::AsAny;
use std::rc::Rc;

use scanner::tokens::Token;

pub enum ElementType {
    Literal,
    Grouping,
    Unary,
    Binary,
}

pub trait ExprT: AsAny {
    fn element_type(&self) -> ElementType;
}

#[derive(PartialEq)]
pub enum StmtType {
    Expression,
    Print,
}
pub trait StmtT: AsAny {
    fn stmt_type(&self) -> StmtType;
    fn is_print(&self) -> bool {
        self.stmt_type() == StmtType::Print
    }
}

pub type StmtRef = Rc<dyn StmtT>;

pub trait Visitor<Ret> {
    fn visit_statement(&self, stmt: StmtRef) -> Ret {
        if stmt.is_print() {
            self.visit_print_stmt(stmt.as_ref().as_any().downcast_ref::<PrintStmt>().unwrap())
        } else {
            self.visit_expression_stmt(stmt.as_ref().as_any().downcast_ref::<ExprStmt>().unwrap())
        }
    }
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Ret;
    fn visit_expression_stmt(&self, stmt: &ExprStmt) -> Ret {
        self.visit_expression(&stmt.value)
    }

    fn visit_expression(&self, expr: &Expression) -> Ret {
        let vall = expr.value.clone();
        return match expr.value.as_ref().element_type() {
            ElementType::Literal => {
                self.visit_literal(vall.as_ref().as_any().downcast_ref::<Literal>().unwrap())
            }
            ElementType::Grouping => {
                self.visit_grouping(vall.as_ref().as_any().downcast_ref::<Grouping>().unwrap())
            }
            ElementType::Unary => {
                self.visit_unary(vall.as_ref().as_any().downcast_ref::<Unary>().unwrap())
            }
            ElementType::Binary => {
                self.visit_binary(vall.as_ref().as_any().downcast_ref::<Binary>().unwrap())
            }
        };
    }

    fn visit_literal(&self, lit: &Literal) -> Ret;
    fn visit_grouping(&self, grp: &Grouping) -> Ret;
    fn visit_unary(&self, unr: &Unary) -> Ret;
    fn visit_binary(&self, bin: &Binary) -> Ret;
}

pub struct ExprStmt {
    pub value: Rc<Expression>,
}
impl StmtT for ExprStmt {
    fn stmt_type(&self) -> StmtType {
        StmtType::Expression
    }
}

pub struct PrintStmt {
    pub value: Rc<Expression>,
}
impl StmtT for PrintStmt {
    fn stmt_type(&self) -> StmtType {
        StmtType::Print
    }
}

// expression

pub struct Expression {
    pub value: Rc<dyn ExprT>,
}

// literals
pub struct Literal {
    pub value: Rc<Token>,
}

impl ExprT for Literal {
    fn element_type(&self) -> ElementType {
        ElementType::Literal
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
}

// unary
pub struct Unary {
    pub operator: Rc<Token>,
    pub expr: Expression,
}

impl ExprT for Unary {
    fn element_type(&self) -> ElementType {
        ElementType::Unary
    }
}

// binary
pub struct Binary {
    pub left: Expression,
    pub operator: Rc<Token>,
    pub right: Expression,
}

impl ExprT for Binary {
    fn element_type(&self) -> ElementType {
        ElementType::Binary
    }
}

pub mod expr_utils {
    use std::rc::Rc;

    use scanner::tokens::{new_token, TokenType};

    use super::{ExprT, Expression, Grouping, Literal};

    pub fn wrap_expr<T: ExprT + 'static>(inner: T) -> Expression {
        Expression {
            value: Rc::new(inner),
        }
    }

    pub fn get_num_literal(num: f64) -> Expression {
        Expression {
            value: Rc::new(Literal {
                value: Rc::new(new_token(TokenType::Number(num))),
            }),
        }
    }

    pub fn group_expr(expr: Expression) -> Expression {
        Expression {
            value: Rc::new(Grouping { expr }),
        }
    }
}
