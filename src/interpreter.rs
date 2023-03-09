use crate::{
    ast::{Binary, Grouping, Literal, Unary, Visitor},
    tokens::TokenType,
};

static INTERPRETER_ERR_TAG: &'static str = "INTERPRETER_ERROR:";
pub struct Interpreter {}

#[derive(Debug)]
pub enum IResult {
    Number(f64),
    String(String),
    Bool(bool),
}

impl Visitor<IResult> for Interpreter {
    fn visit_literal(&self, lit: &Literal) -> IResult {
        match &lit.value.ttype {
            TokenType::String(contents) => IResult::String(contents.to_string()),
            TokenType::Number(value) => IResult::Number(*value),
            TokenType::True => IResult::Bool(true),
            TokenType::False => IResult::Bool(false),
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
                if let IResult::Number(value) = self.visit_expression(&unr.expr) {
                    IResult::Number(minus * value)
                } else {
                    self.error(
                        &unr.operator.ttype,
                        "invalid operand for plus/minus operator",
                    );
                }
            }
            TokenType::Bang => {
                if let IResult::Bool(value) = self.visit_expression(&unr.expr) {
                    IResult::Bool(!value)
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
                if let IResult::Number(left) = leftv {
                    if let IResult::Number(right) = rightv {
                        return IResult::Number(left + right);
                    }
                }
                if let IResult::String(left) = leftv {
                    if let IResult::String(right) = rightv {
                        return IResult::String(format!("{}{}", left, right));
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for plus operator");
            }
            TokenType::Minus => {
                if let IResult::Number(left) = leftv {
                    if let IResult::Number(right) = rightv {
                        return IResult::Number(left - right);
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for minus operator");
            }
            TokenType::Star => {
                if let IResult::Number(left) = leftv {
                    if let IResult::Number(right) = rightv {
                        return IResult::Number(left * right);
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for star operator");
            }
            TokenType::Slash => {
                if let IResult::Number(left) = leftv {
                    if let IResult::Number(right) = rightv {
                        return IResult::Number(left / right);
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for slash operator");
            }
            TokenType::Greater => {
                if let IResult::Number(left) = leftv {
                    if let IResult::Number(right) = rightv {
                        return IResult::Bool(left > right);
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for greater operator");
            }
            TokenType::GreaterEqual => {
                if let IResult::Number(left) = leftv {
                    if let IResult::Number(right) = rightv {
                        return IResult::Bool(left >= right);
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
}

impl Interpreter {
    fn error(&self, ttype: &TokenType, errmsg: &str) -> ! {
        //diverging function
        eprintln!("error for {:?}: {}", ttype, errmsg);
        panic!("{}{}", INTERPRETER_ERR_TAG, errmsg);
    }
}
