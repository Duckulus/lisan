use std::fmt::Display;

#[derive(Debug)]
pub struct ArgumentError {
    expected: String,
    got: usize,
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

pub fn exit(_args: Vec<i32>) -> Result<i32, ArgumentError> {
    std::process::exit(0);
}

pub fn plus(args: Vec<i32>) -> Result<i32, ArgumentError> {
    Ok(args.iter().sum())
}

pub fn minus(args: Vec<i32>) -> Result<i32, ArgumentError> {
    if args.is_empty() {
        Ok(0)
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

pub fn multiply(args: Vec<i32>) -> Result<i32, ArgumentError> {
    if args.is_empty() {
        Ok(0)
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

pub fn square(args: Vec<i32>) -> Result<i32, ArgumentError> {
    if args.len() != 1 {
        Err(ArgumentError {
            expected: String::from("1"),
            got: args.len(),
        })
    } else {
        let num = args.first().unwrap().to_owned();
        Ok(multiply(vec![num, num]).unwrap())
    }
}
