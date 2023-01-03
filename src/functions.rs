use std::{collections::HashMap, fmt::Display};

use crate::OperationMap;

#[derive(Debug)]
pub struct ArgumentError {
    expected: String,
    got: usize,
}

impl ArgumentError {
    fn new(expected: &str, got: usize) -> ArgumentError {
        ArgumentError {
            expected: String::from(expected),
            got,
        }
    }
}

impl Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Wrong number of Arguments. Expected {} but got {}.",
            self.expected, self.got
        )
    }
}

pub fn setup_functions<'a>() -> OperationMap<'a> {
    let mut functions: OperationMap = HashMap::new();
    functions.insert("exit", exit);
    functions.insert("+", plus);
    functions.insert("-", minus);
    functions.insert("*", multiply);
    functions.insert("/", divide);
    functions.insert("square", square);
    functions
}

fn exit(_args: Vec<f32>) -> Result<f32, ArgumentError> {
    std::process::exit(0);
}

fn plus(args: Vec<f32>) -> Result<f32, ArgumentError> {
    Ok(args.iter().sum())
}

fn minus(args: Vec<f32>) -> Result<f32, ArgumentError> {
    if args.is_empty() {
        Err(ArgumentError::new(">=1", args.len()))
    } else if args.len() == 1 {
        Ok(-args.first().unwrap())
    } else {
        let mut result = args.first().unwrap().to_owned();
        for i in args.iter().take(args.len()).skip(1) {
            result -= i;
        }
        Ok(result.to_owned())
    }
}

fn multiply(args: Vec<f32>) -> Result<f32, ArgumentError> {
    if args.is_empty() {
        Ok(0.)
    } else if args.len() == 1 {
        Ok(args.first().unwrap().to_owned())
    } else {
        let mut result = args.first().unwrap().to_owned();
        for i in args.iter().take(args.len()).skip(1) {
            result *= i;
        }
        Ok(result.to_owned())
    }
}

fn divide(args: Vec<f32>) -> Result<f32, ArgumentError> {
    if args.is_empty() {
        Err(ArgumentError::new(">=1", args.len()))
    } else if args.len() == 1 {
        Ok(args.first().unwrap().to_owned())
    } else {
        let mut result = args.first().unwrap().to_owned();
        for i in args.iter().take(args.len()).skip(1) {
            result /= i;
        }
        Ok(result.to_owned())
    }
}

fn square(args: Vec<f32>) -> Result<f32, ArgumentError> {
    if args.len() != 1 {
        Err(ArgumentError::new("1", args.len()))
    } else {
        let num = args.first().unwrap().to_owned();
        Ok(multiply(vec![num, num]).unwrap())
    }
}
