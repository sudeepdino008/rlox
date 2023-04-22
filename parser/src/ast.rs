use as_any::AsAny;
use std::rc::Rc;

use scanner::tokens::TokenRef;

// expression
pub enum ElementType {
    Literal,
    Grouping,
    Unary,
    Binary,
    Assign,
    Logical,
    Call,
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
    Block,
    If,
    While,
    Break,
    Return,
}
pub trait StmtT: DeclT {
    fn stmt_type(&self) -> StmtType;
}

// declaration
pub struct Declaration {
    pub decl: DeclRef,
}

#[derive(PartialEq)]
pub enum DeclType {
    Var,
    Stmt,
    Fun,
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

pub struct FunDecl {
    pub identifier: TokenRef,
    pub params: Vec<TokenRef>,
    pub body: BlockStmt,
}

impl Clone for FunDecl {
    fn clone(&self) -> Self {
        Self {
            identifier: self.identifier.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
        }
    }
}

impl DeclT for FunDecl {
    fn decl_type(&self) -> DeclType {
        DeclType::Fun
    }
    fn as_decl_type(self: Rc<Self>) -> Rc<dyn DeclT> {
        self
    }
}

// statements
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

pub struct BlockStmt {
    pub declarations: Rc<Vec<DeclRef>>,
}

impl Clone for BlockStmt {
    fn clone(&self) -> Self {
        Self {
            declarations: self.declarations.clone(),
        }
    }
}

impl StmtT for BlockStmt {
    fn stmt_type(&self) -> StmtType {
        StmtType::Block
    }
}

pub struct IfStmt {
    pub condition: ExprRef,
    pub then_b: StmtDecl,
    pub else_b: Option<StmtDecl>,
}

impl StmtT for IfStmt {
    fn stmt_type(&self) -> StmtType {
        StmtType::If
    }
}

pub struct WhileStmt {
    pub condition: ExprRef,
    pub body: BlockStmt,
}

impl StmtT for WhileStmt {
    fn stmt_type(&self) -> StmtType {
        StmtType::While
    }
}

pub struct BreakStmt;

impl StmtT for BreakStmt {
    fn stmt_type(&self) -> StmtType {
        StmtType::Break
    }
}

pub struct ReturnStmt {
    pub value: Option<ExprRef>,
}

impl StmtT for ReturnStmt {
    fn stmt_type(&self) -> StmtType {
        StmtType::Return
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

// call
pub struct Call {
    pub callee: Expression,
    pub arguments: Vec<Expression>,
}

impl ExprT for Call {
    fn element_type(&self) -> ElementType {
        ElementType::Call
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

// assignment
pub struct Assign {
    pub identifier: TokenRef,
    pub value: Expression,
}

impl ExprT for Assign {
    fn element_type(&self) -> ElementType {
        ElementType::Assign
    }
}

// Logical expression
pub struct Logical {
    pub left: Expression,
    pub operator: TokenRef,
    pub right: Expression,
}

impl ExprT for Logical {
    fn element_type(&self) -> ElementType {
        ElementType::Logical
    }
}
