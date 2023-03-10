mod errors;

use std::{
    cell::RefCell,
    env,
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Write},
    process::exit,
    rc::Rc,
};

use parser::ast::Binary;
use parser::ast::{expr_utils::*, DeclRef, Unary};
use parser::printer::AstPrinter;
use scanner::tokens::{Token, TokenRef};

use errors::error_handling::ErrorState;
use interpreter::Interpreter;
use parser::ast::Visitor;
use parser::Parser;
use scanner::tokens::TokenType;
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
        print!("\nrlox> ");
        io::stdout().flush().unwrap();
        match io::stdin().lock().read_line(&mut line) {
            Err(why) => {
                eprintln!("{:?}", why);
                continue;
            }
            Ok(_) => {}
        }
        if line.trim().len() == 0 {
            continue;
        }
        run_line(&line);
        set_error(false);
    }
}

#[allow(dead_code)]
fn run_file(filename: &str) {
    let scanner = Scanner::build_scanner(BufReader::new(File::open(filename).unwrap()));
    let mut tokens = Vec::new();
    for lexeme in scanner {
        if lexeme.is_err() {
            eprintln!("stalled due to error");
            //exit_if_error();
            return;
        }
        tokens.push(Rc::new(lexeme.ok().unwrap()));
    }

    parse_tokens(tokens);
    //exit_if_error();
}

fn run_line(contents: &str) {
    let cursor = Cursor::new(contents.as_bytes());
    let scanner = Scanner::build_scanner(BufReader::new(cursor));
    let mut tokens = Vec::new();
    for lexeme in scanner {
        if lexeme.is_err() {
            eprintln!("error in inpur");
            return;
        }
        tokens.push(Rc::new(lexeme.ok().unwrap()));
    }
    match parse_tokens(tokens) {
        Some(stmts) => {
            let result = Interpreter {}.interpret_stmts(stmts);
            match result {
                Ok(result) => println!("{}", result),
                Err(msg) => eprintln!("{}", msg),
            };
        }
        None => {}
    }
}

fn parse_tokens(tokens: Vec<TokenRef>) -> Option<Vec<DeclRef>> {
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(result) => {
            println!("parsed expression: \n");
            let astp = AstPrinter {};
            for stmt in &result {
                println!("{}\n", astp.visit_declaration(stmt.clone()));
            }
            return Some(result);
        }
        Err(_) => {
            //eprint!("error parsing tokens:{}", msg);
            return None;
        }
    }
}

#[allow(dead_code)]
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
    let e1 = get_num_literal(2.0);
    let e2 = get_num_literal(3.0);
    let b1 = wrap_expr(Binary {
        left: e1,
        operator: Rc::new(Token {
            ttype: TokenType::Plus,
            lexeme: "+".to_string(),
            line_num: 0,
        }),
        right: e2,
    });

    let g1 = group_expr(b1);

    let e3 = get_num_literal(4.0);
    let b2 = wrap_expr(Binary {
        left: e3,
        operator: Rc::new(Token {
            ttype: TokenType::Star,
            lexeme: "*".to_string(),
            line_num: 0,
        }),
        right: g1,
    });

    println!("ast: {}", AstPrinter {}.visit_expression(&b2));
    //println!("rpn: {}", RpnPrinter {}.visit_expression(&b2));

    // let's try another
    let e1 = get_num_literal(45.67);
    let g1 = group_expr(e1);

    let e2 = get_num_literal(123.0);
    let e3 = wrap_expr(Unary {
        operator: Rc::new(Token {
            ttype: TokenType::Minus,
            lexeme: "-".to_string(),
            line_num: 0,
        }),
        expr: e2,
    });

    let b2 = wrap_expr(Binary {
        left: g1,
        operator: Rc::new(Token {
            ttype: TokenType::Star,
            lexeme: "*".to_string(),
            line_num: 0,
        }),
        right: e3,
    });
    println!("ast: {}", AstPrinter {}.visit_expression(&b2));
    //println!("rpn: {}", RpnPrinter {}.visit_expression(&b2));
}
