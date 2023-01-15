use std::rc::Rc;

use crate::{
    ast::{Binary, ElementType, Expression, Grouping, Literal, Unary, Visitor},
    tokens::TokenType,
};

pub struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_expression(&self, expr: &Expression) -> String {
        let vall = Rc::clone(&expr.value);
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
}

impl AstPrinter {
    fn parenthesize(&self, exprs: Vec<String>) -> String {
        let mut content = String::new();
        content.push_str("(");

        for expr in exprs {
            content.push_str(expr.as_str());
            content.push_str(" ");
        }

        content.push_str(")");
        return content;
    }
}
