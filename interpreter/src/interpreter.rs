use std::{
    fmt::Display,
    panic::{self, AssertUnwindSafe},
};

use parser::ast::{self, Binary, Grouping, Literal, Unary, Visitor};
use scanner::tokens::TokenType;

use IResult::{Bool, None, Number, String};

static INTERPRETER_ERR_TAG: &'static str = "INTERPRETER_ERROR:";
pub struct Interpreter {}

pub enum IResult {
    Number(f64),
    String(std::string::String),
    Bool(bool),
    None,
}

impl Display for IResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(n) => write!(f, "{}", n),
            String(s) => write!(f, "{}", s),
            Bool(b) => write!(f, "{}", b),
            None => write!(f, ""),
        }
    }
}

impl Visitor<IResult> for Interpreter {
    fn visit_literal(&self, lit: &Literal) -> IResult {
        match &lit.value.ttype {
            TokenType::String(contents) => String(contents.to_string()),
            TokenType::Number(value) => Number(*value),
            TokenType::True => Bool(true),
            TokenType::False => Bool(false),
            tkn => self.error(&tkn, "invalid token found; expected literal"),
        }
    }

    fn visit_grouping(&self, grp: &Grouping) -> IResult {
        self.visit_expression(&grp.expr)
    }

    fn visit_unary(&self, unr: &Unary) -> IResult {
        match &unr.operator.ttype {
            TokenType::Minus | TokenType::Plus => {
                let minus = if unr.operator.ttype == TokenType::Minus {
                    -1.0
                } else {
                    1.0
                };
                if let Number(value) = self.visit_expression(&unr.expr) {
                    Number(minus * value)
                } else {
                    self.error(
                        &unr.operator.ttype,
                        "invalid operand for plus/minus operator",
                    );
                }
            }
            TokenType::Bang => {
                if let Bool(value) = self.visit_expression(&unr.expr) {
                    Bool(!value)
                } else {
                    self.error(&unr.operator.ttype, "invalid operand for bang operator");
                }
            }
            tkn => self.error(&tkn, "invalid token found; expected unary operator"),
        }
    }

    fn visit_binary(&self, bin: &Binary) -> IResult {
        let leftv = self.visit_expression(&bin.left);
        let rightv = self.visit_expression(&bin.right);
        match &bin.operator.ttype {
            TokenType::Plus => {
                if let Number(left) = leftv {
                    if let Number(right) = rightv {
                        return Number(left + right);
                    }
                }
                if let String(left) = leftv {
                    if let String(right) = rightv {
                        return String(format!("{}{}", left, right));
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for plus operator");
            }
            TokenType::Minus => {
                if let Number(left) = leftv {
                    if let Number(right) = rightv {
                        return Number(left - right);
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for minus operator");
            }
            TokenType::Star => {
                if let Number(left) = leftv {
                    if let Number(right) = rightv {
                        return Number(left * right);
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for star operator");
            }
            TokenType::Slash => {
                if let Number(left) = leftv {
                    if let Number(right) = rightv {
                        return Number(left / right);
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for slash operator");
            }
            TokenType::Greater => {
                if let Number(left) = leftv {
                    if let Number(right) = rightv {
                        return Bool(left > right);
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for greater operator");
            }
            TokenType::GreaterEqual => {
                if let Number(left) = leftv {
                    if let Number(right) = rightv {
                        return Bool(left >= right);
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for greater-equal")
            }
            TokenType::LeftParen => todo!(),
            TokenType::RightParen => todo!(),
            TokenType::LeftBrace => todo!(),
            TokenType::RightBrace => todo!(),
            TokenType::Comma => todo!(),
            TokenType::Dot => todo!(),
            TokenType::Semicolon => todo!(),
            TokenType::Bang => todo!(),
            TokenType::BangEqual => todo!(),
            TokenType::Equal => todo!(),
            TokenType::EqualEqual => todo!(),
            TokenType::Less => todo!(),
            TokenType::LessEqual => todo!(),
            TokenType::Identifier => todo!(),
            TokenType::String(_) => todo!(),
            TokenType::Number(_) => todo!(),
            TokenType::And => todo!(),
            TokenType::Class => todo!(),
            TokenType::Else => todo!(),
            TokenType::False => todo!(),
            TokenType::Fun => todo!(),
            TokenType::For => todo!(),
            TokenType::If => todo!(),
            TokenType::Nil => todo!(),
            TokenType::Or => todo!(),
            TokenType::Print => todo!(),
            TokenType::Return => todo!(),
            TokenType::Super => todo!(),
            TokenType::This => todo!(),
            TokenType::True => todo!(),
            TokenType::Var => todo!(),
            TokenType::While => todo!(),
            TokenType::Eof => todo!(),
        }
    }

    fn visit_print_stmt(&self, stmt: &ast::PrintStmt) -> IResult {
        println!("{}", self.visit_expression(&stmt.value));
        return None;
    }

    fn visit_var_decl(&self, decl: &ast::VarDecl) -> IResult {
        todo!()
    }
}

impl Interpreter {
    pub fn interpret_stmts(
        &self,
        stmts: Vec<ast::StmtRef>,
    ) -> Result<IResult, std::string::String> {
        let mut result = IResult::None;
        for stmt in stmts {
            match self.interpret_stmt(stmt) {
                Ok(val) => result = val,
                Err(err) => return Err(err),
            }
        }
        return Ok(result);
    }
    pub fn interpret_stmt(&self, stmt: ast::StmtRef) -> Result<IResult, std::string::String> {
        let prev = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if let Some(s) = info.payload().downcast_ref::<std::string::String>() {
                if s.starts_with(INTERPRETER_ERR_TAG) {
                    eprintln!("{s:?}");
                    return;
                }
            }
            prev(info);
        }));

        let result = panic::catch_unwind(AssertUnwindSafe(|| self.visit_statement(stmt)));
        return if let Ok(exp) = result {
            Ok(exp)
        } else {
            Err("".to_string())
        };
    }

    fn error(&self, _ttype: &TokenType, errmsg: &str) -> ! {
        //diverging function
        //eprintln!("error for {:?}: {}", ttype, errmsg);
        panic!("{}{}", INTERPRETER_ERR_TAG, errmsg);
    }
}
