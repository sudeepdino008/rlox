use std::any::Any;

use crate::ast::{Binary, ElementType, ExprT, Expression, Grouping, Literal, Unary, Visitor};

struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_expression(&self, expr: &Expression) -> String {
        self.visit(
            expr.value.as_ref().element_type(),
            Box::new(&expr.value.as_ref() as &dyn Any),
        )
    }

    fn visit_literal(&self, lit: &Literal) -> String {
        lit.value.lexeme.clone()
    }

    fn visit_grouping(&self, grp: &Grouping) -> String {
        let mut content = String::new();
        content.push_str("(");
        content.push_str(&self.visit_expression(&grp.expr));
        content.push_str(")");

        return content;
    }

    fn visit_unary(&self, unr: &Unary) -> String {
        let mut content = String::new();
        content.push_str(&unr.operator.lexeme);
        content.push_str(&self.visit_expression(&unr.expr));
        return content;
    }

    fn visit_binary(&self, bin: &Binary) -> String {
        let mut content = String::new();
        content.push_str(&self.visit_expression(&bin.left));
        content.push_str(&bin.operator.lexeme);
        content.push_str(&self.visit_expression(&bin.right));
        return content;
    }
}

impl AstPrinter {
    fn parenthesize(name: String, exprs: &[Box<dyn ExprT>]) -> String {
        return "".to_string();
    }

    fn visit(&self, element_type: ElementType, expr: Box<dyn Any>) -> String {
        match element_type {
            Expression => self.visit_expression(expr.downcast_ref::<Expression>().unwrap()),
            Literal => self.visit_literal(expr.downcast_ref::<Literal>().unwrap()),
            Grouping => self.visit_grouping(expr.downcast_ref::<Grouping>().unwrap()),
            Unary => self.visit_unary(expr.downcast_ref::<Unary>().unwrap()),
            Binary => self.visit_binary(expr.downcast_ref::<Binary>().unwrap()),
        };

        "".to_string()
    }
}
