use std::rc::Rc;

use crate::{
    ast::{self, Binary, Grouping, Literal, Unary},
    utils::Visitor,
};
use scanner::tokens::TokenType;

pub struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_literal(&mut self, lit: &Literal) -> String {
        match &lit.value.ttype {
            TokenType::String(contents) => contents.to_string(),
            TokenType::Number(num) => num.to_string(),
            _ => lit.value.lexeme.clone(),
        }
    }

    fn visit_grouping(&mut self, grp: &Grouping) -> String {
        let exprs = vec!["group".to_string(), self.visit_expression(&grp.expr)];
        self.parenthesize(exprs)
    }

    fn visit_unary(&mut self, unr: &Unary) -> String {
        let exprs = vec![
            unr.operator.lexeme.clone(),
            self.visit_expression(&unr.expr),
        ];
        self.parenthesize(exprs)
    }

    fn visit_binary(&mut self, bin: &Binary) -> String {
        let exprs = vec![
            bin.operator.lexeme.clone(),
            self.visit_expression(&bin.left),
            self.visit_expression(&bin.right),
        ];
        self.parenthesize(exprs)
    }

    fn visit_print_stmt(&mut self, stmt: &ast::PrintStmt) -> String {
        let exprs = vec![
            "print".to_string(),
            "\"".to_string(),
            self.visit_expression(&stmt.value),
            "\"".to_string(),
        ];
        self.parenthesize(exprs)
    }

    fn visit_var_decl(&mut self, decl: &ast::VarDecl) -> String {
        let mut exprs = Vec::new();
        exprs.push("var".to_string());
        exprs.push(decl.identifier.lexeme.clone());
        if let Some(expr) = &decl.rhs {
            exprs.push("=".to_string());
            exprs.push(self.visit_expression(expr));
        }

        self.parenthesize(exprs)
    }

    fn visit_assign(&mut self, assign: &ast::Assign) -> String {
        let exprs = vec![
            assign.identifier.lexeme.clone(),
            "=".to_string(),
            self.visit_expression(&assign.value),
        ];
        self.parenthesize(exprs)
    }

    fn visit_block_stmt(&mut self, stmt: &ast::BlockStmt) -> String {
        let mut exprs = Vec::new();
        exprs.push("block{".to_string());
        for decl in stmt.declarations.iter() {
            exprs.push(self.visit_declaration(decl.clone()));
        }
        exprs.push("}".to_string());
        self.parenthesize(exprs)
    }

    fn visit_if_stmt(&mut self, stmt: &ast::IfStmt) -> String {
        let mut exprs = vec![
            "if".to_string(),
            self.visit_expression(&stmt.condition),
            "then\n".to_string(),
            self.visit_statement(&stmt.then_b),
        ];

        if let Some(else_br) = &stmt.else_b {
            exprs.push("\nelse\n".to_string());
            exprs.push(self.visit_statement(else_br));
            exprs.push("\nend\n".to_string());
        }

        self.parenthesize(exprs)
    }

    fn visit_logical(&mut self, logic: &ast::Logical) -> String {
        let exprs = vec![
            self.visit_expression(&logic.left),
            logic.operator.lexeme.clone(),
            self.visit_expression(&logic.right),
        ];
        self.parenthesize(exprs)
    }

    fn visit_while_stmt(&mut self, stmt: &ast::WhileStmt) -> String {
        let exprs = vec![
            "while".to_string(),
            self.visit_expression(&stmt.condition),
            "\n{\n".to_string(),
            self.visit_block_stmt(&stmt.body),
            "\n{\n".to_string(),
        ];
        self.parenthesize(exprs)
    }

    fn visit_break_stmt(&mut self, _stmt: &ast::BreakStmt) -> String {
        self.parenthesize(vec!["break".to_string()])
    }

    fn visit_call(&mut self, call: &ast::Call) -> String {
        let mut exprs = vec![self.visit_expression(&call.callee), "(".to_string()];
        for arg in call.arguments.iter() {
            exprs.push(self.visit_expression(arg));
        }
        exprs.push(")".to_string());
        self.parenthesize(exprs)
    }

    fn visit_fun_decl(&mut self, decl: Rc<ast::FunDecl>) -> String {
        let mut exprs = vec![
            "fun".to_string(),
            decl.identifier.lexeme.clone(),
            "(".to_string(),
        ];
        for param in decl.params.iter() {
            exprs.push(param.lexeme.clone());
        }

        exprs.push(")".to_string());
        exprs.push("{\n".to_string());
        exprs.push(self.visit_block_stmt(&decl.body));
        exprs.push("\n}".to_string());
        self.parenthesize(exprs)
    }
}

impl AstPrinter {
    fn parenthesize(&self, exprs: Vec<String>) -> String {
        let mut content = String::new();
        content.push('(');

        for expr in exprs {
            content.push(' ');
            content.push_str(expr.as_str());
        }

        content.push(')');
        content
    }
}
