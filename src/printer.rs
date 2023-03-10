use crate::{
    ast::{self, Binary, Grouping, Literal, Unary, Visitor},
    tokens::TokenType,
};

pub struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_literal(&self, lit: &Literal) -> String {
        match &lit.value.ttype {
            TokenType::String(contents) => contents.to_string(),
            TokenType::Number(num) => num.to_string(),
            _ => lit.value.lexeme.clone(),
        }
    }

    fn visit_grouping(&self, grp: &Grouping) -> String {
        let mut exprs = Vec::new();
        exprs.push("group".to_string());
        exprs.push(self.visit_expression(&grp.expr));
        self.parenthesize(exprs)
    }

    fn visit_unary(&self, unr: &Unary) -> String {
        let mut exprs = Vec::new();
        exprs.push(unr.operator.lexeme.clone());
        exprs.push(self.visit_expression(&unr.expr));
        self.parenthesize(exprs)
    }

    fn visit_binary(&self, bin: &Binary) -> String {
        let mut exprs = Vec::new();
        exprs.push(bin.operator.lexeme.clone());
        exprs.push(self.visit_expression(&bin.left));
        exprs.push(self.visit_expression(&bin.right));
        self.parenthesize(exprs)
    }

    fn visit_print_stmt(&self, stmt: &ast::PrintStmt) -> String {
        let mut exprs = Vec::new();
        exprs.push("print".to_string());
        exprs.push(self.visit_expression(&stmt.value));
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

// Reverse Polish Notation printer

pub struct RpnPrinter {}

impl Visitor<String> for RpnPrinter {
    fn visit_literal(&self, lit: &Literal) -> String {
        match &lit.value.ttype {
            TokenType::String(contents) => contents.to_string(),
            TokenType::Number(num) => num.to_string(),
            _ => lit.value.lexeme.clone(),
        }
    }

    fn visit_grouping(&self, grp: &Grouping) -> String {
        let mut exprs = Vec::new();
        exprs.push(self.visit_expression(&grp.expr));
        self.stringify(exprs)
    }

    fn visit_unary(&self, unr: &Unary) -> String {
        let mut exprs = Vec::new();
        exprs.push(unr.operator.lexeme.clone());
        exprs.push(self.visit_expression(&unr.expr));
        self.stringify(exprs)
    }

    fn visit_binary(&self, bin: &Binary) -> String {
        let mut exprs = Vec::new();
        exprs.push(self.visit_expression(&bin.left));
        exprs.push(self.visit_expression(&bin.right));
        exprs.push(bin.operator.lexeme.clone());
        self.stringify(exprs)
    }

    fn visit_print_stmt(&self, stmt: &ast::PrintStmt) -> String {
        let mut exprs = Vec::new();
        exprs.push(self.visit_expression(&stmt.value));
        exprs.push("print".to_string());
        self.stringify(exprs)
    }
}

impl RpnPrinter {
    fn stringify(&self, exprs: Vec<String>) -> String {
        let mut content = String::new();

        for expr in exprs {
            content.push_str(expr.as_str());
            content.push_str(" ");
        }

        return content;
    }
}
