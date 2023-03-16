use as_any::AsAny;
use std::rc::Rc;

use scanner::tokens::TokenRef;

// expression
pub enum ElementType {
    Literal,
    Grouping,
    Unary,
    Binary,
}

pub trait ExprT: AsAny {
    fn element_type(&self) -> ElementType;
}

pub type ExprRef = Rc<Expression>;

// statements
#[derive(PartialEq)]
pub enum StmtType {
    Expression,
    Print,
}
pub trait StmtT: DeclT {
    fn stmt_type(&self) -> StmtType;
    fn is_print(&self) -> bool {
        self.stmt_type() == StmtType::Print
    }
}

// declaration
pub struct Declaration {
    pub decl: DeclRef,
}

#[derive(PartialEq)]
pub enum DeclType {
    Var,
    Stmt,
}

pub trait DeclT: AsAny {
    fn decl_type(&self) -> DeclType;
    fn is_var(&self) -> bool {
        self.decl_type() == DeclType::Var
    }
    fn as_decl_type(self: Rc<Self>) -> Rc<dyn DeclT>;
}

impl<T> DeclT for T
where
    T: StmtT,
{
    fn decl_type(&self) -> DeclType {
        DeclType::Stmt
    }

    fn as_decl_type(self: Rc<Self>) -> Rc<dyn DeclT> {
        self
    }
}

pub type DeclRef = Rc<dyn DeclT>;

pub struct VarDecl {
    pub identifier: TokenRef,
    pub rhs: Option<ExprRef>,
}

impl VarDecl {
    pub fn new(identifier: TokenRef) -> Self {
        Self {
            identifier,
            rhs: None,
        }
    }

    pub fn new_with_assign(identifier: TokenRef, assign: ExprRef) -> Self {
        Self {
            identifier,
            rhs: Some(assign),
        }
    }
}

impl DeclT for VarDecl {
    fn decl_type(&self) -> DeclType {
        DeclType::Var
    }
    fn as_decl_type(self: Rc<Self>) -> Rc<dyn DeclT> {
        self
    }
}

pub struct StmtDecl {
    // unfortunately, we can't use StmtRef directly
    // here because trait object can't be "upcasted" into each other easily
    pub stmt: Rc<dyn StmtT>,
}

impl DeclT for StmtDecl {
    fn decl_type(&self) -> DeclType {
        DeclType::Stmt
    }
    fn as_decl_type(self: Rc<Self>) -> Rc<dyn DeclT> {
        self
    }
}

impl From<Rc<dyn DeclT>> for StmtDecl {
    fn from(decl: Rc<dyn DeclT>) -> StmtDecl {
        StmtDecl {
            stmt: decl
                .as_any()
                .downcast_ref::<StmtDecl>()
                .unwrap()
                .stmt
                .clone(),
        }
    }
}

// visitor trait
pub trait Visitor<Ret> {
    fn visit_declaration(&mut self, decl: DeclRef) -> Ret {
        if decl.is_var() {
            self.visit_var_decl(decl.as_ref().as_any().downcast_ref::<VarDecl>().unwrap())
        } else {
            self.visit_statement(decl.as_ref().as_any().downcast_ref::<StmtDecl>().unwrap())
        }
    }
    fn visit_var_decl(&mut self, decl: &VarDecl) -> Ret;
    fn visit_statement(&mut self, stmt: &StmtDecl) -> Ret {
        if stmt.stmt.is_print() {
            self.visit_print_stmt(
                stmt.stmt
                    .as_ref()
                    .as_any()
                    .downcast_ref::<PrintStmt>()
                    .unwrap(),
            )
        } else {
            self.visit_expression_stmt(
                stmt.stmt
                    .as_ref()
                    .as_any()
                    .downcast_ref::<ExprStmt>()
                    .unwrap(),
            )
        }
    }
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Ret;
    fn visit_expression_stmt(&mut self, stmt: &ExprStmt) -> Ret {
        self.visit_expression(&stmt.value)
    }

    fn visit_expression(&mut self, expr: &Expression) -> Ret {
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

    fn visit_literal(&mut self, lit: &Literal) -> Ret;
    fn visit_grouping(&mut self, grp: &Grouping) -> Ret;
    fn visit_unary(&mut self, unr: &Unary) -> Ret;
    fn visit_binary(&mut self, bin: &Binary) -> Ret;
}

pub struct ExprStmt {
    pub value: ExprRef,
}
impl StmtT for ExprStmt {
    fn stmt_type(&self) -> StmtType {
        StmtType::Expression
    }
}

pub struct PrintStmt {
    pub value: ExprRef,
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
    pub value: TokenRef,
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
    pub operator: TokenRef,
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
    pub operator: TokenRef,
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
