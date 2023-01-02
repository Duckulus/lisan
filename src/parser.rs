use crate::{functions::ArgumentError, scanner::Token, OperationMap};

#[derive(Debug, Clone)]
pub struct Expression {
    operator: Option<String>,
    operands: Vec<ExpressionValue>,
}

#[derive(Debug, Clone)]
enum ExpressionValue {
    Value(i32),
    SubExpression(Expression),
}

impl Expression {
    fn new() -> Expression {
        Expression {
            operator: None,
            operands: Vec::new(),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Option<Expression> {
    let mut root_expression: Option<Expression> = None;
    let mut expression_stack = Vec::new();
    for token in &tokens {
        match token {
            Token::ExpressionStart => {
                let expression = Expression::new();
                expression_stack.push(expression);
            }
            Token::ExpressionEnd => {
                let popped = expression_stack.pop();
                if !expression_stack.is_empty() {
                    let mut top_copy = expression_stack.last().unwrap().clone();
                    expression_stack.pop();
                    top_copy
                        .operands
                        .push(ExpressionValue::SubExpression(popped.unwrap()));
                    expression_stack.push(top_copy);
                } else {
                    root_expression = Some(popped.unwrap());
                }
            }
            Token::Function(op) => {
                let mut top_copy = expression_stack.last().unwrap().clone();
                expression_stack.pop();
                top_copy.operator = Some(String::from(op));

                expression_stack.push(top_copy);
            }
            Token::Symbol(value) => {
                let mut top_copy = expression_stack.last().unwrap().clone();
                expression_stack.pop();
                top_copy
                    .operands
                    .push(ExpressionValue::Value(value.to_owned()));
                expression_stack.push(top_copy);
            }
        }
    }
    root_expression
}

pub fn evaluate(expression: &Expression, functions: &OperationMap) -> Result<i32, ArgumentError> {
    let mut args: Vec<i32> = Vec::new();
    for operand in &expression.operands {
        args.push(evaluate_iter(operand, functions).unwrap());
    }
    functions
        .get(expression.operator.as_ref().unwrap().as_str())
        .unwrap()(args)
}

fn evaluate_iter(value: &ExpressionValue, functions: &OperationMap) -> Result<i32, ArgumentError> {
    match value {
        ExpressionValue::Value(x) => Ok(x.to_owned()),
        ExpressionValue::SubExpression(exp) => evaluate(exp, functions),
    }
}
