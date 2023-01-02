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

fn main() {
    println!("{} v{}", PACKAGE_NAME, PACKAGE_VERSION);
    let mut functions: HashMap<&str, fn(Vec<i32>) -> Result<i32, ArgumentError>> = HashMap::new();
    functions.insert("+", functions::plus);
    functions.insert("-", functions::minus);

    repl(&functions)
}

fn repl(functions: &HashMap<&str, fn(Vec<i32>) -> Result<i32, ArgumentError>>) {
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        println!("{}", eval(read().as_str(), functions))
    }
}

fn read() -> String {
    let mut expression = String::new();
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read input expression");
    expression
}

fn eval(
    expression: &str,
    functions: &HashMap<&str, fn(Vec<i32>) -> Result<i32, ArgumentError>>,
) -> i32 {
    let tokens = scanner::tokenize(expression).unwrap();
    let exp = parser::parse(tokens).unwrap();
    parser::evaluate(&exp, functions)
}
