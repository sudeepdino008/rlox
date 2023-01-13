mod ast;
mod errors;
mod scanner;
mod tokens;
mod utils;

use std::{
    cell::RefCell,
    env,
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Write},
    process::exit,
    rc::Rc,
};

use crate::ast::Literal;
use crate::ast::{Expression, Grouping};
use crate::utils::AstPrinter;
use crate::{ast::Binary, tokens::Token};

use crate::ast::Visitor;
use crate::tokens::{new_token, TokenType};
use errors::error_handling::ErrorState;
use scanner::Scanner;

thread_local!(static ERROR_STATE: RefCell<ErrorState>  = RefCell::new(ErrorState { error_occured: false }));

fn main() {
    //try_ast_printer();

    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // start the interpreter mode
        run_prompt();
    } else if args.len() == 2 {
        let filename = &args[1];
        run_file(filename);
    } else {
        eprintln!("Usage: rlox [filename]");
    }
}

#[allow(dead_code)]
fn run_prompt() {
    loop {
        // simply moving this line outside the loop will append to this "line" variable and not just store the current input
        let mut line = String::new();
        print!("rlox> ");
        io::stdout().flush().unwrap();
        match io::stdin().lock().read_line(&mut line) {
            Err(why) => {
                eprintln!("{:?}", why);
                continue;
            }
            Ok(_) => {}
        }
        println!("the line is: {}", line);
        run_line(&line);
        set_error(false);
    }
}

#[allow(dead_code)]
fn run_file(filename: &str) {
    let scanner = Scanner::build_scanner(BufReader::new(File::open(filename).unwrap()));
    for lexeme in scanner {
        println!("{:?}", lexeme);
    }
    exit_if_error();
}

fn run_line(contents: &str) {
    let cursor = Cursor::new(contents.as_bytes());
    let scanner = Scanner::build_scanner(BufReader::new(cursor));
    for lexeme in scanner {
        println!("{:?}", lexeme);
    }
    println!("well came this far!");
}

fn exit_if_error() {
    ERROR_STATE.with(|val| {
        if val.borrow().error_occured {
            exit(65);
        }
    });
}

fn set_error(is_error: bool) {
    ERROR_STATE.with(|val| {
        val.borrow_mut().error_occured = is_error;
    })
}

/// AST printer testing

#[allow(dead_code)]
fn try_ast_printer() {
    //     expression     → literal
    //                | unary
    //                | binary
    //                | grouping ;

    // literal        → NUMBER | STRING | "true" | "false" | "nil" ;
    // grouping       → "(" expression ")" ;
    // unary          → ( "-" | "!" ) expression ;
    // binary         → expression operator expression ;
    // operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
    //                | "+"  | "-"  | "*" | "/" ;
    // 2 + 3

    let e1 = get_num_literal(2.0);
    let e2 = get_num_literal(3.0);
    let b1 = wrap_expr(Binary {
        left: e1,
        operator: Token {
            ttype: TokenType::Plus,
            lexeme: "+".to_string(),
            line_num: 0,
        },
        right: e2,
    });

    let g1 = Expression {
        value: Rc::new(Grouping { expr: b1 }),
    };

    let e3 = get_num_literal(4.0);
    let b2 = wrap_expr(Binary {
        left: e3,
        operator: Token {
            ttype: TokenType::Star,
            lexeme: "*".to_string(),
            line_num: 0,
        },
        right: g1,
    });

    let printer = AstPrinter {};
    println!("{}", printer.visit_expression(&b2));
}

fn get_num_literal(num: f64) -> Expression {
    Expression {
        value: Rc::new(Literal {
            value: new_token(TokenType::Number(num)),
        }),
    }
}

fn wrap_expr(bin: Binary) -> Expression {
    Expression {
        value: Rc::new(bin),
    }
}
