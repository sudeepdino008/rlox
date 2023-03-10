pub mod ast;
pub mod printer;

use std::{
    panic::{self, AssertUnwindSafe},
    rc::Rc,
};

use crate::ast::{
    expr_utils::wrap_expr, Binary, ExprStmt, Expression, Grouping, Literal, PrintStmt, StmtRef,
    Unary,
};

use scanner::tokens::{Token, TokenType};

static PARSER_ERR_TAG: &'static str = "PARSER_ERROR:";

#[allow(dead_code)]
pub struct Parser {
    pub tokens: Vec<Rc<Token>>,
    token_cursor: usize,
}

/*
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;


program -> statement* EOF ;
statement -> exprStmt | printStmt ;
exprStmt -> expression ";" ;
printStmt -> "print" expression ";" ;

*/

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Rc<Token>>) -> Parser {
        Parser {
            tokens,
            token_cursor: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<StmtRef>, String> {
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
                stmts.push(self.statement());
            }
            return stmts;
        })) {
            Ok(stmts) => Ok(stmts),
            Err(_) => Err("".to_string()),
        }
    }

    pub fn statement(&mut self) -> StmtRef {
        if self.match_t(&[TokenType::Print]) {
            Rc::new(self.print_stmt())
        } else {
            Rc::new(self.expr_stmt())
        }
    }

    pub fn print_stmt(&mut self) -> PrintStmt {
        let value = self.expression();
        self.consume(&TokenType::Semicolon, "semicolon missing");
        PrintStmt {
            value: Rc::new(value),
        }
    }

    pub fn expr_stmt(&mut self) -> ExprStmt {
        let value = self.expression();
        self.consume(&TokenType::Semicolon, "semicolon missing");
        ExprStmt {
            value: Rc::new(value),
        }
    }

    pub fn expression(&mut self) -> Expression {
        return self.equality();
    }

    pub fn equality(&mut self) -> Expression {
        self.binary_break(
            |p: &mut Parser| p.comparison(),
            &[TokenType::EqualEqual, TokenType::BangEqual],
        )
    }

    pub fn comparison(&mut self) -> Expression {
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

    pub fn term(&mut self) -> Expression {
        self.binary_break(
            |p: &mut Parser| p.factor(),
            &[TokenType::Minus, TokenType::Plus],
        )
    }

    pub fn factor(&mut self) -> Expression {
        self.binary_break(
            |p: &mut Parser| p.unary(),
            &[TokenType::Slash, TokenType::Star],
        )
    }

    pub fn unary(&mut self) -> Expression {
        if self.match_t(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let expr = self.unary();
            wrap_expr(Unary { operator, expr })
        } else {
            self.primary()
        }
    }

    pub fn primary(&mut self) -> Expression {
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
        ]) {
            let expr = self.previous();
            wrap_expr(Literal { value: expr })
        } else {
            let token = self.previous();
            self.error(&token.ttype, "literal expected");
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

    fn match_t(&mut self, tokens_types: &[TokenType]) -> bool {
        for ttype in tokens_types {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn advance(&mut self) {
        self.token_cursor += 1;
    }

    fn check(&self, ttype: &TokenType) -> bool {
        !self.is_end()
            && std::mem::discriminant(&self.peek().ttype) == std::mem::discriminant(ttype)
    }

    fn peek(&self) -> Rc<Token> {
        self.tokens[self.token_cursor].clone()
    }

    fn is_end(&self) -> bool {
        self.token_cursor >= self.tokens.len() - 1
    }

    fn previous(&self) -> Rc<Token> {
        self.tokens[self.token_cursor - 1].clone()
    }

    fn consume(&mut self, ttype: &TokenType, errmsg: &str) {
        if self.check(&ttype) {
            return self.advance();
        }

        self.error(&ttype, errmsg);
    }

    fn error(&mut self, _ttype: &TokenType, errmsg: &str) -> ! {
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
