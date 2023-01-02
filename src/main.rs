use std::collections::HashMap;

use crate::functions::ArgumentError;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

mod functions;

fn main() {
    println!("{} v{}", PACKAGE_NAME, PACKAGE_VERSION);
    let mut functions: HashMap<&str, fn(Vec<i32>) -> Result<i32, ArgumentError>> = HashMap::new();
    functions.insert("+", functions::plus);
    functions.insert("-", functions::minus);

    println!("{}", eval("(+ 3 3 3 3)", functions));
}

mod parser;
mod scanner;

fn eval(
    expression: &str,
    functions: HashMap<&str, fn(Vec<i32>) -> Result<i32, ArgumentError>>,
) -> i32 {
    let tokens = scanner::tokenize(expression).unwrap();
    let mut exp = parser::parse(tokens).unwrap();
    parser::evaluate(&mut exp, &functions)
}
