use crate::ast::{self, Binary, Grouping, Literal, Unary, Visitor};
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
        let mut exprs = Vec::new();
        exprs.push("group".to_string());
        exprs.push(self.visit_expression(&grp.expr));
        self.parenthesize(exprs)
    }

    fn visit_unary(&mut self, unr: &Unary) -> String {
        let mut exprs = Vec::new();
        exprs.push(unr.operator.lexeme.clone());
        exprs.push(self.visit_expression(&unr.expr));
        self.parenthesize(exprs)
    }

    fn visit_binary(&mut self, bin: &Binary) -> String {
        let mut exprs = Vec::new();
        exprs.push(bin.operator.lexeme.clone());
        exprs.push(self.visit_expression(&bin.left));
        exprs.push(self.visit_expression(&bin.right));
        self.parenthesize(exprs)
    }

    fn visit_print_stmt(&mut self, stmt: &ast::PrintStmt) -> String {
        let mut exprs = Vec::new();
        exprs.push("print".to_string());
        exprs.push("\"".to_string());
        exprs.push(self.visit_expression(&stmt.value));
        exprs.push("\"".to_string());
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
}

impl AstPrinter {
    fn parenthesize(&self, exprs: Vec<String>) -> String {
        let mut content = String::new();
        content.push_str("(");

        for expr in exprs {
            content.push_str(" ");
            content.push_str(expr.as_str());
        }

        content.push_str(")");
        return content;
    }
}
