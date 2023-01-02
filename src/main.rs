const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("{} v{}", PACKAGE_NAME, PACKAGE_VERSION);
    eval("(+ 1 2 (+ 3 4))");
}

mod parser;
mod scanner;

fn eval(expression: &str) -> &str {
    let tokens = scanner::tokenize(expression).unwrap();
    let exp = parser::parse(tokens);
    dbg!(exp);
    expression
}
