use std::rc::Rc;

use crate::ast::{
    Assign, Binary, BlockStmt, BreakStmt, Call, DeclRef, DeclType, ElementType, ExprStmt,
    Expression, FunDecl, Grouping, IfStmt, Literal, Logical, PrintStmt, StmtDecl, StmtType, Unary,
    VarDecl, WhileStmt,
};

// visitor trait
pub trait Visitor<Ret> {
    fn visit_declaration(&mut self, decl: DeclRef) -> Ret {
        match decl.decl_type() {
            DeclType::Var => {
                self.visit_var_decl(decl.as_ref().as_any().downcast_ref::<VarDecl>().unwrap())
            }
            DeclType::Stmt => {
                self.visit_statement(decl.as_ref().as_any().downcast_ref::<StmtDecl>().unwrap())
            }
            DeclType::Fun => self.visit_fun_decl(Rc::new(
                decl.as_ref()
                    .as_any()
                    .downcast_ref::<FunDecl>()
                    .unwrap()
                    .to_owned(),
            )),
        }
    }
    fn visit_var_decl(&mut self, decl: &VarDecl) -> Ret;
    fn visit_fun_decl(&mut self, decl: Rc<FunDecl>) -> Ret;
    fn visit_statement(&mut self, stmt: &StmtDecl) -> Ret {
        match stmt.stmt.stmt_type() {
            StmtType::Expression => {
                self.visit_expression_stmt(stmt.stmt.as_ref().as_any().downcast_ref().unwrap())
            }
            StmtType::Print => {
                self.visit_print_stmt(stmt.stmt.as_ref().as_any().downcast_ref().unwrap())
            }
            StmtType::Block => {
                self.visit_block_stmt(stmt.stmt.as_ref().as_any().downcast_ref().unwrap())
            }
            StmtType::If => self.visit_if_stmt(stmt.stmt.as_ref().as_any().downcast_ref().unwrap()),
            StmtType::While => {
                self.visit_while_stmt(stmt.stmt.as_ref().as_any().downcast_ref().unwrap())
            }
            StmtType::Break => {
                self.visit_break_stmt(stmt.stmt.as_ref().as_any().downcast_ref().unwrap())
            }
        }
    }
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Ret;
    fn visit_expression_stmt(&mut self, stmt: &ExprStmt) -> Ret {
        self.visit_expression(&stmt.value)
    }
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Ret;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Ret;
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Ret;
    fn visit_break_stmt(&mut self, stmt: &BreakStmt) -> Ret;

    fn visit_expression(&mut self, expr: &Expression) -> Ret {
        let vall = expr.value.clone();
        return match expr.value.as_ref().element_type() {
            ElementType::Literal => {
                self.visit_literal(vall.as_ref().as_any().downcast_ref().unwrap())
            }
            ElementType::Grouping => {
                self.visit_grouping(vall.as_ref().as_any().downcast_ref().unwrap())
            }
            ElementType::Unary => self.visit_unary(vall.as_ref().as_any().downcast_ref().unwrap()),
            ElementType::Binary => {
                self.visit_binary(vall.as_ref().as_any().downcast_ref().unwrap())
            }
            ElementType::Assign => {
                self.visit_assign(vall.as_ref().as_any().downcast_ref().unwrap())
            }
            ElementType::Logical => {
                self.visit_logical(vall.as_ref().as_any().downcast_ref().unwrap())
            }
            ElementType::Call => self.visit_call(vall.as_ref().as_any().downcast_ref().unwrap()),
        };
    }

    fn visit_literal(&mut self, lit: &Literal) -> Ret;
    fn visit_grouping(&mut self, grp: &Grouping) -> Ret;
    fn visit_unary(&mut self, unr: &Unary) -> Ret;
    fn visit_binary(&mut self, bin: &Binary) -> Ret;
    fn visit_logical(&mut self, logic: &Logical) -> Ret;
    fn visit_assign(&mut self, assign: &Assign) -> Ret;
    fn visit_call(&mut self, call: &Call) -> Ret;
}

pub mod expr_utils {
    use std::rc::Rc;

    use scanner::tokens::{new_token, TokenType};

    use crate::ast::ExprT;

    use super::{Expression, Grouping, Literal};

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
