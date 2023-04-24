mod callable;
mod environment;
mod foreignf;
mod result;

use std::io::{stdout, Stdout, Write};

use std::panic::{self, AssertUnwindSafe};
use std::rc::Rc;

use callable::{EnvironmentAware, LoxCallable, VisitorEnvironmentAware};
use environment::{Environment, EnvironmentRef};
use parser::ast::{self, Binary, FunDecl, Grouping, Literal, Unary};
use parser::utils::Visitor;

use rustcore::Shared;
use scanner::tokens::TokenType;

use result::IResult;
use result::IResult::{Bool, None, Number, String};

static INTERPRETER_ERR_TAG: &str = "INTERPRETER_ERROR:";

pub struct Interpreter<T: Write> {
    environment: EnvironmentRef,
    ostream: Shared<T>,
}

impl<T: Write> EnvironmentAware for Interpreter<T> {
    fn get_environment(&self) -> EnvironmentRef {
        self.environment.clone()
    }

    fn set_environment(&mut self, environment: EnvironmentRef) {
        self.environment = environment;
    }
}

impl<T: Write> VisitorEnvironmentAware for Interpreter<T> {}

impl<T: Write> Visitor<IResult> for Interpreter<T> {
    fn visit_var_decl(&mut self, decl: &ast::VarDecl) -> IResult {
        if decl.rhs.is_none() {
            self.environment
                .borrow_mut()
                .declare(&decl.identifier.lexeme);
        } else {
            let rhs_result = self.visit_expression(decl.rhs.as_ref().unwrap().as_ref());
            self.environment
                .borrow_mut()
                .declare_and_init(&decl.identifier.lexeme, rhs_result);
        }

        None
    }

    fn visit_print_stmt(&mut self, stmt: &ast::PrintStmt) -> IResult {
        let value = self.visit_expression(&stmt.value);
        //self.ostream.by_ref()
        //let mut ss = self.ostream;
        match writeln!(self.ostream.borrow_mut(), "{}", value) {
            Ok(_) => {}
            Err(err) => self.error(
                &TokenType::Print,
                format!("failed to write to output stream: {:?}", err).as_str(),
            ),
        }
        None
    }

    fn visit_block_stmt(&mut self, stmt: &ast::BlockStmt) -> IResult {
        let parent: EnvironmentRef = self.environment.clone();
        self.environment = Environment::new_with_parent(parent.clone());
        let mut result = None;

        for decl in stmt.declarations.iter() {
            let decl_result = self.visit_declaration(decl.clone());
            match decl_result {
                IResult::Break => {
                    result = IResult::Break;
                    break;
                }
                IResult::Return(val) => {
                    result = val.as_ref().clone();
                    break;
                }

                _ => {
                    continue;
                }
            }
        }

        self.environment = parent;

        result
    }

    fn visit_if_stmt(&mut self, stmt: &ast::IfStmt) -> IResult {
        let condition_val = self.visit_expression(&stmt.condition);
        if let Bool(condition) = condition_val {
            return if condition {
                self.visit_statement(&stmt.then_b)
            } else if let Some(else_b) = &stmt.else_b {
                self.visit_statement(else_b)
            } else {
                None
            };
        } else {
            self.error(&TokenType::If, "condition expression should return boolean")
        }
    }

    fn visit_while_stmt(&mut self, stmt: &ast::WhileStmt) -> IResult {
        let condition_val = self.visit_expression(&stmt.condition);
        if let Bool(condition) = condition_val {
            if condition {
                if self.visit_block_stmt(&stmt.body) != IResult::Break {
                    self.visit_while_stmt(stmt);
                }
            }
        } else {
            self.error(
                &TokenType::While,
                "condition expression should return boolean",
            )
        }

        None
    }

    fn visit_break_stmt(&mut self, _stmt: &ast::BreakStmt) -> IResult {
        IResult::Break
    }

    fn visit_literal(&mut self, lit: &Literal) -> IResult {
        match &lit.value.ttype {
            TokenType::String(contents) => String(Rc::new(contents.to_string())),
            TokenType::Number(value) => Number(*value),
            TokenType::True => Bool(true),
            TokenType::False => Bool(false),
            TokenType::Identifier => {
                let var = &lit.value.lexeme;
                if let Some(value) = self.environment.get(var) {
                    if IResult::None.eq(value.as_ref()) {
                        self.error(&lit.value.ttype, "variable is not initialized");
                    }

                    value.as_ref().clone()
                } else {
                    self.error(
                        &lit.value.ttype,
                        format!("variable {} not in scope", var.as_str()).as_str(),
                    )
                }
            }
            tkn => self.error(tkn, "invalid token found; expected literal"),
        }
    }

    fn visit_grouping(&mut self, grp: &Grouping) -> IResult {
        self.visit_expression(&grp.expr)
    }

    fn visit_unary(&mut self, unr: &Unary) -> IResult {
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
            tkn => self.error(tkn, "invalid token found; expected unary operator"),
        }
    }

    fn visit_binary(&mut self, bin: &Binary) -> IResult {
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
                        return String(Rc::new(format!("{}{}", left, right)));
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
            TokenType::BangEqual => Bool(leftv != rightv),
            TokenType::Equal => todo!(),
            TokenType::EqualEqual => Bool(leftv == rightv),
            TokenType::Less => {
                if let Number(left) = leftv {
                    if let Number(right) = rightv {
                        return Bool(left < right);
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for less")
            }
            TokenType::LessEqual => {
                if let Number(left) = leftv {
                    if let Number(right) = rightv {
                        return Bool(left <= right);
                    }
                }
                self.error(&bin.operator.ttype, "invalid operands for less-equal")
            }
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
            TokenType::Break => todo!(),
        }
    }

    fn visit_logical(&mut self, logic: &ast::Logical) -> IResult {
        let leftv = self.visit_expression(&logic.left);

        if let Bool(leftvb) = leftv {
            if leftvb == (logic.operator.ttype == TokenType::Or) {
                // short circuit
                return Bool(leftvb);
            } else {
                let rightv = self.visit_expression(&logic.right);
                if let Bool(rightvb) = rightv {
                    return Bool(rightvb);
                } else {
                    self.error(&TokenType::Or, "condition expression should return boolean")
                }
            }
        } else {
            self.error(
                &logic.operator.ttype,
                "invalid operands for logical operator",
            );
        }
    }

    fn visit_assign(&mut self, assign: &ast::Assign) -> IResult {
        let identifier = assign.identifier.lexeme.as_str();
        if self.environment.is_binded(identifier) {
            let rhs = self.visit_expression(&assign.value);
            self.environment.borrow_mut().assign(identifier, rhs);
            None
        } else {
            self.error(
                &assign.identifier.ttype,
                format!("{} is not binded", identifier).as_str(),
            );
        }
    }

    fn visit_call(&mut self, call: &ast::Call) -> IResult {
        let callee = self.visit_expression(&call.callee);
        if let IResult::Callable(arg0) = callee {
            let arguments = call
                .arguments
                .iter()
                .map(|a| self.visit_expression(a))
                .collect::<Vec<IResult>>();
            if arguments.len() != arg0.arity {
                self.error(
                    &TokenType::Identifier,
                    format!(
                        "expected {} arguments but got {}",
                        arg0.arity,
                        arguments.len()
                    )
                    .as_str(),
                );
            }
            (arg0.borrow_mut().call)(self as &mut dyn VisitorEnvironmentAware, arguments)
        } else {
            self.error(
                &TokenType::Identifier,
                format!("{} is not callable", callee).as_str(),
            );
        }
    }

    fn visit_fun_decl(&mut self, decl: Rc<FunDecl>) -> IResult {
        // variables and function names share the same namespace
        let identifier = decl.identifier.lexeme.as_str();
        let decl = decl.clone();
        let callable = LoxCallable {
            arity: decl.params.len(),
            call: Box::new(move |visitor, params: Vec<IResult>| {
                // bind the variables
                // call the function body
                // return the result
                let parent = visitor.get_environment();
                visitor.set_environment(Environment::new_with_parent(parent.clone()));

                let mut viter = params.iter();
                for param in decl.params.iter() {
                    visitor
                        .get_environment()
                        .declare_and_init(param.lexeme.as_str(), viter.next().unwrap().clone())
                }

                let result = visitor.visit_block_stmt(&decl.body);
                visitor.set_environment(parent);
                result
            }),
        };
        self.environment
            .borrow_mut()
            .declare_and_init(identifier, IResult::Callable(Shared::new(callable)));

        None
    }

    fn visit_return_stmt(&mut self, stmt: &ast::ReturnStmt) -> IResult {
        IResult::Return(Rc::new(if let Some(value) = &stmt.value {
            self.visit_expression(&value)
        } else {
            None
        }))
    }
}

impl Default for Interpreter<Stdout> {
    fn default() -> Self {
        Interpreter::new()
    }
}

impl Interpreter<Stdout> {
    fn new() -> Interpreter<Stdout> {
        Interpreter::new_with_out(Shared::new(stdout()))
    }
}

impl<T: Write> Interpreter<T> {
    pub fn new_with_out(ostream: Shared<T>) -> Interpreter<T> {
        Interpreter {
            environment: Environment::new(),
            ostream,
        }
    }

    pub fn interpret(&mut self, decls: Vec<ast::DeclRef>) -> Result<IResult, std::string::String> {
        let mut result = IResult::None;
        for decl in decls {
            match self.interpret_decl(decl) {
                Ok(IResult::Break) => return Err("break outside loop".to_string()),
                Err(err) => return Err(err),
                Ok(val) => result = val,
            }
        }
        Ok(result)
    }
    fn interpret_decl(&mut self, decl: ast::DeclRef) -> Result<IResult, std::string::String> {
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

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            let result = self.visit_declaration(decl);
            if let IResult::Break = result {
                self.error(&TokenType::Break, "break outside loop");
            }
            result
        }));
        if let Ok(exp) = result {
            Ok(exp)
        } else {
            //Err("".to_string())
            Err(result
                .unwrap_err()
                .downcast::<std::string::String>()
                .unwrap()
                .to_string())
        }
    }

    fn error(&self, _ttype: &TokenType, errmsg: &str) -> ! {
        //diverging function
        //eprintln!("error for {:?}: {}", ttype, errmsg);
        panic!("{}{}", INTERPRETER_ERR_TAG, errmsg);
    }
}
