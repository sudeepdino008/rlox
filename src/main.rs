mod errors;
mod scanner;
mod tokens;

use std::{
    cell::RefCell,
    env,
    fs::{self, File},
    io::{self, BufRead, BufReader, Cursor, Write},
    process::exit,
};

use errors::error_handling::ErrorState;
use scanner::Scanner;

thread_local!(static ERROR_STATE: RefCell<ErrorState>  = RefCell::new(ErrorState { error_occured: false }));

fn main() {
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
