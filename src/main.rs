mod errors;

use std::{
    cell::RefCell,
    env,
    fs::File,
    io::{self, BufRead, BufReader, Cursor, Read, Seek, Stdout, Write},
    process::exit,
    rc::Rc,
};


use parser::ast::{DeclRef};

use scanner::tokens::{TokenRef};

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
    let mut interpreter = Interpreter::new();
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
        if line.trim().is_empty() {
            continue;
        }
        run_line(&mut interpreter, &line);
        set_error(false);
    }
}

#[allow(dead_code)]
fn run_file(filename: &str) {
    let mut interpreter = Interpreter::new();
    let scanner = Scanner::build_scanner(BufReader::new(File::open(filename).unwrap()));
    execute(&mut interpreter, scanner);
}

fn run_line(interpreter: &mut Interpreter<Stdout>, contents: &str) {
    let cursor = Cursor::new(contents.as_bytes());
    let scanner = Scanner::build_scanner(BufReader::new(cursor));
    execute(interpreter, scanner);

    // let mut tokens = Vec::new();
    // for lexeme in scanner {
    //     if lexeme.is_err() {
    //         eprintln!("error in inpur");
    //         return;
    //     }
    //     tokens.push(Rc::new(lexeme.ok().unwrap()));
    // }
    // match parse_tokens(tokens) {
    //     Some(decls) => {
    //         let result = interpreter.interpret(decls);
    //         match result {
    //             Ok(result) => println!("{}", result),
    //             Err(msg) => eprintln!("{}", msg),
    //         };
    //     }
    //     None => {}
    // }
}

fn execute<T: Read + Seek, I: Write>(interpreter: &mut Interpreter<I>, scanner: Scanner<T>) {
    let mut tokens = Vec::new();
    for lexeme in scanner {
        if lexeme.is_err() {
            eprintln!("error in inpur");
            return;
        }
        tokens.push(Rc::new(lexeme.ok().unwrap()));
    }
    match parse_tokens(tokens) {
        Some(decls) => {
            let result = interpreter.interpret(decls);
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
            // println!("parsed expression: \n");
            // let mut astp = AstPrinter {};
            // for stmt in &result {
            //     println!("{}\n", astp.visit_declaration(stmt.clone()));
            // }
            Some(result)
        }
        Err(_) => {
            //eprint!("error parsing tokens:{}", msg);
            None
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

#[cfg(test)]
mod tests {
    use std::{fs};

    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn data1() {
        let input_file = "data/1/vars.rl";
        let exp_file = "data/1/expected";
        let expected_out = fs::read_to_string(exp_file).unwrap();

        let scanner = Scanner::build_scanner(BufReader::new(File::open(input_file).unwrap()));
        let cursor = Rc::new(RefCell::new(Cursor::new(Vec::new())));

        // writeln!(boxed, "hello world").unwrap();
        // boxed
        //     .as_mut()
        //     .seek(std::io::SeekFrom::Start(0))
        //     .expect("expected to seek just fine");

        // let mut out2 = String::new();
        // let res = boxed.as_mut().read_to_string(&mut out2);
        // println!("res: {:?}", res);
        // println!("out2: {}", out2);

        let mut interpreter = Interpreter::new_with_out(cursor.clone());
        execute(&mut interpreter, scanner);

        //println!("prevp: {:?}", boxed.as_mut().stream_position());

        cursor
            .borrow_mut()
            .seek(std::io::SeekFrom::Start(0))
            .expect("expected to seek just fine");

        let mut out = String::new();
        cursor
            .borrow_mut()
            .read_to_string(&mut out)
            .expect("read didn't go as expected");

        assert_eq!(out, expected_out);
    }
}
