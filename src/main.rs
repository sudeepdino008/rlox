use std::{env, fs, io::{self, Write}};

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
    let mut line: String = "".to_string();
    loop {
        print!("rlox> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut line) {
            Err(why) => {
                eprintln!("{:?}", why);
                continue;
            },
            Ok(_) => (),
        }
        run(&line);
    }
}

fn run_file(filename: &str) {
    let contents = fs::read_to_string(filename).expect("can't read file contents");
    run(&contents);
}

fn run(contents: &str) {
    println!("well came this far!");
}
