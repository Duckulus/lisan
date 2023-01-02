#[derive(Debug)]
pub struct ArgumentError {
    expected: String,
    got: i32,
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
