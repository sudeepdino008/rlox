pub mod ast;
pub mod printer;
pub mod utils;

use std::{
    panic::{self, AssertUnwindSafe},
    rc::Rc,
};

use crate::ast::{
    Binary, Call, ExprStmt, Expression, Grouping, Literal, Logical, PrintStmt, Unary,
};

use utils::expr_utils::wrap_expr;

use ast::{Assign, BlockStmt, BreakStmt, DeclRef, FunDecl, IfStmt, StmtDecl, VarDecl, WhileStmt};
use scanner::tokens::{TokenRef, TokenType};

static PARSER_ERR_TAG: &str = "PARSER_ERROR:";

#[allow(dead_code)]
pub struct Parser {
    pub tokens: Vec<TokenRef>,
    token_cursor: usize,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<TokenRef>) -> Parser {
        Parser {
            tokens,
            token_cursor: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<DeclRef>, String> {
        // special handling based on parser error tag
        let prev = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if let Some(s) = info.payload().downcast_ref::<String>() {
                if s.starts_with(PARSER_ERR_TAG) {
                    eprintln!("{s:?}");
                    return;
                }
            }
            prev(info);
        }));

        match panic::catch_unwind(AssertUnwindSafe(|| {
            let mut stmts = Vec::new();
            while !self.is_end() {
                stmts.push(self.declaration());
            }
            stmts
        })) {
            Ok(stmts) => Ok(stmts),
            Err(_) => Err("".to_string()),
        }
    }

    fn declaration(&mut self) -> DeclRef {
        if self.match_t(&[TokenType::Var]) {
            Rc::new(self.var_declaration())
        } else if self.match_t(&[TokenType::Fun]) {
            Rc::new(self.fun_declaration())
        } else {
            Rc::new(self.statement()) //.as_decl_type()
        }
    }

    fn fun_declaration(&mut self) -> FunDecl {
        // 'fun' is already matched
        if self.match_t(&[TokenType::Identifier]) {
            let identifier = self.previous();
            self.consume(&TokenType::LeftParen, "expected '(' after function name");
            let mut params = Vec::new();
            if !self.match_t(&[TokenType::RightParen]) {
                loop {
                    if params.len() >= 255 {
                        self.error("cannot have more than 255 parameters");
                    }
                    if self.match_t(&[TokenType::Identifier]) {
                        params.push(self.previous());
                    }
                    if self.match_t(&[TokenType::Comma]) {
                        continue;
                    }
                    // comma not found, should end with right paren
                    self.consume(
                        &TokenType::RightParen,
                        "expected paranthesis after parameters",
                    );
                    break;
                }
            }

            let bstmt = self.block_stmt(false);
            FunDecl {
                identifier,
                params,
                body: bstmt,
            }
        } else {
            self.error("expected identifier after 'fun'")
        }
    }

    fn var_declaration(&mut self) -> VarDecl {
        if self.match_t(&[TokenType::Identifier]) {
            let identifier = self.previous();
            let mut rhs = None;
            if self.match_t(&[TokenType::Equal]) {
                let initializer = self.expression();
                rhs = Some(Rc::new(initializer))
            }
            self.consume(&TokenType::Semicolon, "semicolon missing");
            return VarDecl { identifier, rhs };
        }
        self.error("expected identifier after 'var'")
    }

    fn statement(&mut self) -> StmtDecl {
        StmtDecl {
            stmt: if self.match_t(&[TokenType::Print]) {
                Rc::new(self.print_stmt())
            } else if self.match_t(&[TokenType::LeftParen]) {
                Rc::new(self.block_stmt(true))
            } else if self.match_t(&[TokenType::If]) {
                Rc::new(self.if_stmt())
            } else if self.match_t(&[TokenType::While]) {
                Rc::new(self.while_stmt())
            } else if self.match_t(&[TokenType::Break]) {
                Rc::new(self.break_stmt())
            } else {
                Rc::new(self.expr_stmt())
            },
        }
    }

    fn break_stmt(&mut self) -> BreakStmt {
        // break has been matched
        self.consume(&TokenType::Semicolon, "semicolon missing");
        BreakStmt {}
    }

    fn print_stmt(&mut self) -> PrintStmt {
        let value = self.expression();
        self.consume(&TokenType::Semicolon, "semicolon missing");
        PrintStmt {
            value: Rc::new(value),
        }
    }

    fn expr_stmt(&mut self) -> ExprStmt {
        let value = self.expression();
        self.consume(&TokenType::Semicolon, "semicolon missing");
        ExprStmt {
            value: Rc::new(value),
        }
    }

    fn block_stmt(&mut self, ft_consumed: bool) -> BlockStmt {
        if !ft_consumed {
            self.consume(&TokenType::LeftParen, "expected '{' at start of block");
        }
        let mut decls = Vec::new();
        while !self.match_t(&[TokenType::RightParen]) {
            decls.push(self.declaration());
        }
        BlockStmt {
            declarations: Rc::new(decls),
        }
    }

    fn if_stmt(&mut self) -> IfStmt {
        // assuming if is already consumed
        IfStmt {
            condition: Rc::new(self.expression()),
            then_b: self.statement(),
            else_b: if self.match_t(&[TokenType::Else]) {
                Some(self.statement())
            } else {
                None
            },
        }
    }

    fn while_stmt(&mut self) -> WhileStmt {
        // assuming while is already consumed
        WhileStmt {
            condition: Rc::new(self.expression()),
            body: self.block_stmt(false),
        }
    }

    fn expression(&mut self) -> Expression {
        self.assignment()
    }

    fn assignment(&mut self) -> Expression {
        if self.match_t(&[TokenType::Identifier]) {
            let identifier = self.previous();
            if self.match_t(&[TokenType::Equal]) {
                let value = self.assignment();
                return wrap_expr(Assign { identifier, value });
            }

            self.retreat();
        }

        self.logic_or()
    }

    fn logic_or(&mut self) -> Expression {
        self.logic_break(|p: &mut Parser| p.logic_and(), &[TokenType::Or])
    }

    fn logic_and(&mut self) -> Expression {
        self.logic_break(|p: &mut Parser| p.equality(), &[TokenType::And])
    }

    fn equality(&mut self) -> Expression {
        self.binary_break(
            |p: &mut Parser| p.comparison(),
            &[TokenType::EqualEqual, TokenType::BangEqual],
        )
    }

    fn comparison(&mut self) -> Expression {
        self.binary_break(
            |p: &mut Parser| p.term(),
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
        )
    }

    fn term(&mut self) -> Expression {
        self.binary_break(
            |p: &mut Parser| p.factor(),
            &[TokenType::Minus, TokenType::Plus],
        )
    }

    fn factor(&mut self) -> Expression {
        self.binary_break(
            |p: &mut Parser| p.unary(),
            &[TokenType::Slash, TokenType::Star],
        )
    }

    fn unary(&mut self) -> Expression {
        if self.match_t(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let expr = self.unary();
            wrap_expr(Unary { operator, expr })
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Expression {
        let mut expr = self.primary();
        if self.match_t(&[TokenType::LeftBrace]) {
            let arguments = self.arguments();
            self.consume(&TokenType::RightBrace, "expected ')' after arguments");
            expr = wrap_expr(Call {
                callee: expr,
                arguments,
            });
        }

        expr
    }

    fn arguments(&mut self) -> Vec<Expression> {
        let mut args = Vec::new();
        if !self.match_t(&[TokenType::RightBrace]) {
            loop {
                args.push(self.expression());
                if !self.match_t(&[TokenType::Comma]) {
                    break;
                }
                if args.len() > 255 {
                    self.error("too many arguments");
                }
            }
        }

        args
    }

    fn primary(&mut self) -> Expression {
        if self.match_t(&[TokenType::LeftBrace]) {
            let expr = self.expression();
            self.consume(&TokenType::RightBrace, "right brace missing");
            return wrap_expr(Grouping { expr });
        }

        if self.match_t(&[
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::String("".to_string()),
            TokenType::Number(0.0),
            TokenType::Identifier,
        ]) {
            let expr = self.previous();
            wrap_expr(Literal { value: expr })
        } else {
            self.error("literal expected");
        }
    }

    fn binary_break(
        &mut self,
        gen: fn(&mut Parser) -> Expression,
        token_types: &[TokenType],
    ) -> Expression {
        let mut expr = gen(self);

        while self.match_t(token_types) {
            let operator = self.previous();
            let right_expr = gen(self);
            expr = wrap_expr(Binary {
                left: expr,
                operator,
                right: right_expr,
            });
        }

        expr
    }

    fn logic_break(
        &mut self,
        gen: fn(&mut Parser) -> Expression,
        token_types: &[TokenType],
    ) -> Expression {
        let mut expr = gen(self);

        while self.match_t(token_types) {
            let operator = self.previous();
            let right_expr = gen(self);
            expr = wrap_expr(Logical {
                left: expr,
                operator,
                right: right_expr,
            });
        }

        expr
    }

    fn match_t(&mut self, tokens_types: &[TokenType]) -> bool {
        for ttype in tokens_types {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn advance(&mut self) {
        self.token_cursor += 1;
    }

    fn retreat(&mut self) {
        self.token_cursor -= 1;
    }

    fn check(&self, ttype: &TokenType) -> bool {
        !self.is_end()
            && std::mem::discriminant(&self.peek().ttype) == std::mem::discriminant(ttype)
    }

    fn peek(&self) -> TokenRef {
        self.tokens[self.token_cursor].clone()
    }

    fn is_end(&self) -> bool {
        self.token_cursor >= self.tokens.len() - 1
    }

    fn previous(&self) -> TokenRef {
        self.tokens[self.token_cursor - 1].clone()
    }

    fn consume(&mut self, ttype: &TokenType, errmsg: &str) {
        if self.check(ttype) {
            return self.advance();
        }

        self.error(errmsg);
    }

    fn error(&mut self, errmsg: &str) -> ! {
        //diverging function
        //eprintln!("error for {:?}: {}", ttype, errmsg);
        panic!("{}{}", PARSER_ERR_TAG, errmsg);
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_end() {
            if self.previous().ttype == TokenType::Semicolon {
                return;
            }

            match self.peek().ttype {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}
