mod errors;
mod tests;

use std::{
    cell::RefCell,
    env,
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Read, Seek, Stdout, Write},
    process::exit,
    rc::Rc,
};

use parser::ast::DeclRef;

use scanner::tokens::TokenRef;

use errors::error_handling::ErrorState;
use interpreter::Interpreter;

use parser::Parser;

use scanner::Scanner;

thread_local!(static ERROR_STATE: RefCell<ErrorState>  = RefCell::new(ErrorState { error_occured: false }));

fn main() {
    //try_ast_printer();
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
    let mut interpreter = Interpreter::default();
    loop {
        // simply moving this line outside the loop will append to this "line" variable and not just store the current input
        let mut line = String::new();
        print!("\nrlox> ");
        io::stdout().flush().unwrap();
        if let Err(why) = io::stdin().lock().read_line(&mut line) {
            eprintln!("{:?}", why);
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }
        run_line(&mut interpreter, &line);
        set_error(false);
    }
}

#[allow(dead_code)]
fn run_file(filename: &str) {
    let mut interpreter = Interpreter::default();
    let scanner = Scanner::build_scanner(BufReader::new(File::open(filename).unwrap()));
    _ = execute(&mut interpreter, scanner);
}

fn run_line(interpreter: &mut Interpreter<Stdout>, contents: &str) {
    let cursor = Cursor::new(contents.as_bytes());
    let scanner = Scanner::build_scanner(BufReader::new(cursor));
    _ = execute(interpreter, scanner);
}

fn execute<T: Read + Seek, I: Write>(
    interpreter: &mut Interpreter<I>,
    scanner: Scanner<T>,
) -> Result<(), String> {
    let mut tokens = Vec::new();
    for lexeme in scanner {
        if lexeme.is_err() {
            eprintln!("error in inpur");
            return Err(lexeme.unwrap_err());
        }
        tokens.push(Rc::new(lexeme.ok().unwrap()));
        //println!("{:?}", tokens.last().unwrap());
    }
    if let Some(decls) = parse_tokens(tokens) {
        // // print parser output
        // for decl in decls.iter() {
        //     AstPrinter {}.visit_declaration(decl.clone());
        // }

        let result = interpreter.interpret(decls);
        match result {
            Ok(result) => {
                println!("{}", result);
                Ok(())
            }
            Err(msg) => {
                //eprintln!("{}", msg)
                Err(msg)
            }
        }
    } else {
        Err("error in parsing tokens".to_string())
    }
}

fn parse_tokens(tokens: Vec<TokenRef>) -> Option<Vec<DeclRef>> {
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(result) => Some(result),
        Err(_) => None,
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
