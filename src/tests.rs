use std::{
    fs::{self, File},
    io::{BufReader, Cursor, Read, Seek},
};

use interpreter::Interpreter;
use rustcore::Shared;
use scanner::Scanner;

struct RunParams {
    should_fail: bool,
}

#[test]
fn successful_interpreter_runs() {
    let params = &RunParams { should_fail: false };
    compare_interpreter_runs("data/1/input.rl", "data/1/expected.txt", params);
    compare_interpreter_runs("data/2/input.rl", "data/2/expected.txt", params);
}

#[test]
fn out_of_scope_err() {
    let params = &RunParams { should_fail: true };
    compare_interpreter_runs("data/3/input.rl", "data/3/expected.txt", params);
}

#[test]
fn if_tests() {
    let mut params = &mut RunParams { should_fail: false };
    compare_interpreter_runs("data/4/input.rl", "data/4/expected.txt", params);

    params.should_fail = true;
    compare_interpreter_runs("data/4/input_err.rl", "data/4/expected_err.txt", params);
}

#[test]
fn logical_operators() {
    let params = &RunParams { should_fail: false };
    compare_interpreter_runs("data/5/input.rl", "data/5/expected.txt", params);
}

#[test]
fn while_loop() {
    let mut params = &mut RunParams { should_fail: false };
    compare_interpreter_runs("data/6/input.rl", "data/6/expected.txt", params);

    params.should_fail = true;
    compare_interpreter_runs("data/6/input_err.rl", "data/6/expected_err.txt", params);
}

#[test]
fn test_break() {
    let mut params = &mut RunParams { should_fail: false };
    compare_interpreter_runs("data/7/input.rl", "data/7/expected.txt", params);
    compare_interpreter_runs(
        "data/7/input_nested.rl",
        "data/7/expected_nested.txt",
        params,
    );

    params.should_fail = true;
    compare_interpreter_runs("data/7/input_err.rl", "data/7/expected_err.txt", params);
}

#[test]
fn functions() {
    let params = &RunParams { should_fail: false };
    compare_interpreter_runs("data/8/input.rl", "data/8/expected.txt", params);
    compare_interpreter_runs(
        "data/8/input_local_fn.rl",
        "data/8/expected_local_fn.txt",
        params,
    );
}

// #[test]
// fn recursion() {
//     let params = &RunParams { should_fail: false };
//     compare_interpreter_runs("data/8/input.rl", "data/8/expected.txt", params);
// }

// #[test]
// fn class() {
//     let params = &RunParams { should_fail: false };
//     compare_interpreter_runs("data/9/input.rl", "data/9/expected.txt", params);
// }

// #[test]
// fn inheritance() {
//     let params = &RunParams { should_fail: false };
//     compare_interpreter_runs("data/10/input.rl", "data/10/expected.txt", params);
// }

// #[test]
// fn methods() {
//     let params = &RunParams { should_fail: false };
//     compare_interpreter_runs("data/11/input.rl", "data/11/expected.txt", params);
// }

// #[test]
// fn init() {
//     let params = &RunParams { should_fail: false };
//     compare_interpreter_runs("data/12/input.rl", "data/12/expected.txt", params);
// }

// #[test]
// fn getter() {
//     let params = &RunParams { should_fail: false };
//     compare_interpreter_runs("data/13/input.rl", "data/13/expected.txt", params);
// }

// #[test]
// fn setter() {
//     let params = &RunParams { should_fail: false };
//     compare_interpreter_runs("data/14/input.rl", "data/14/expected.txt", params);
// }

// #[test]
// fn this() {
//     let params = &RunParams { should_fail: false };
//     compare_interpreter_runs("data/15/input.rl", "data/15/expected.txt", params);
// }

// #[test]
// fn super_() {
//     let params = &RunParams { should_fail: false };
//     compare_interpreter_runs("data/16/input.rl", "data/16/expected.txt", params);
// }

// tests for if statements
// tests for out of scope errors
// tests for if statements

// tests for logical operators
// while loop (with breaks)
// functions and calls

#[allow(dead_code)]
fn compare_interpreter_runs(input_program: &str, expected_out_file: &str, params: &RunParams) {
    let expected_out = fs::read_to_string(expected_out_file).unwrap();
    let scanner = Scanner::build_scanner(BufReader::new(File::open(input_program).unwrap()));
    let cursor = Shared::new(Cursor::new(Vec::new()));

    let mut interpreter = Interpreter::new_with_out(cursor.clone());
    let result = crate::execute(&mut interpreter, scanner);

    if params.should_fail {
        if let Err(msg) = result {
            assert_eq!(msg, expected_out);
        } else {
            assert!(false, "expected to get error from interpreter");
        }

        return;
    }

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
