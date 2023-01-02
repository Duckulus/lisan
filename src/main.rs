use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::functions::ArgumentError;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

mod functions;
mod parser;
mod scanner;

type OperationMap<'a> = HashMap<&'a str, fn(Vec<i32>) -> Result<i32, ArgumentError>>;

fn main() {
    println!("{} v{}", PACKAGE_NAME, PACKAGE_VERSION);
    let mut functions: OperationMap = HashMap::new();
    functions.insert("+", functions::plus);
    functions.insert("-", functions::minus);
    functions.insert("*", functions::multiply);
    functions.insert("square", functions::square);

    repl(&functions)
}

fn repl(functions: &OperationMap) {
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        match eval(read().as_str(), functions) {
            Ok(value) => println!("{}", value),
            Err(err) => eprintln!("{}", err),
        }
    }
}

fn read() -> String {
    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read input expression");
    expression
}

fn eval(expression: &str, functions: &OperationMap) -> Result<i32, ArgumentError> {
    let tokens = scanner::tokenize(expression).unwrap();
    let exp = parser::parse(tokens).unwrap();
    parser::evaluate(&exp, functions)
}
