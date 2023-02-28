use std::rc::Rc;

use crate::{
    ast::{Binary, ExprT, ExprUtils::wrap_expr, Expression},
    tokens::{Token, TokenType},
};

#[allow(dead_code)]
pub(crate) struct Parser {
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
*/

impl Parser {
    pub fn new(tokens: Vec<Rc<Token>>) -> Parser {
        Parser {
            tokens,
            token_cursor: 0,
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
    */

    pub fn term(&mut self) -> Expression {
        self.binary_break(
            |p: &mut Parser| p.factor(),
            &[TokenType::Minus, TokenType::Plus],
        )
    }

    pub fn factor(&mut self) -> Expression {
        todo!()
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
            if self.check(ttype.clone()) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn advance(&mut self) {
        self.token_cursor += self.token_cursor + 1;
    }

    fn check(&self, ttype: TokenType) -> bool {
        !self.is_end() && self.peek().ttype == ttype
    }

    fn peek(&self) -> Rc<Token> {
        self.tokens[self.token_cursor].clone()
    }

    fn is_end(&self) -> bool {
        self.token_cursor >= self.tokens.len()
    }

    fn previous(&self) -> Rc<Token> {
        self.tokens[self.token_cursor - 1].clone()
    }
}
