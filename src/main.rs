use std::env;
use colored::Colorize;

mod interpreter;

fn main() {
    let arg = env::args().nth(1).expect("try 'honeycomb -h' for help");
    if arg == "-h" {
        println!("honeycomb [file]: interpret a file as a honeycomb program");
        println!("honeycomb -h: show this help message");
        return;
    }
    let mut interpreter = interpreter::Interpreter::new(arg);
    match interpreter.execute() {
        Ok(_) => (),
        Err(e) => println!("{}: {}", "Error".red(), e),
    }

}