use std::{
    cell::RefCell,
    fs::{self, File},
    io::{BufReader, Cursor, Read, Seek},
    rc::Rc,
};

use interpreter::Interpreter;
use scanner::Scanner;

#[test]
fn interpreter_runs() {
    compare_interpreter_runs("data/1/vars.rl", "data/1/expected");
}

fn compare_interpreter_runs(input_program: &str, expected_out_file: &str) {
    let expected_out = fs::read_to_string(expected_out_file).unwrap();
    let scanner = Scanner::build_scanner(BufReader::new(File::open(input_program).unwrap()));
    let cursor = Rc::new(RefCell::new(Cursor::new(Vec::new())));

    let mut interpreter = Interpreter::new_with_out(cursor.clone());
    crate::execute(&mut interpreter, scanner);

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
