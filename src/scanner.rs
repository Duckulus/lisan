use std::fmt::Error;

#[derive(Debug)]
pub enum Token {
    ExpressionStart,
    ExpressionEnd,
    Function(String),
    Symbol(f32),
}

pub fn tokenize(expression: &str) -> Result<Vec<Token>, Error> {
    enum Expected {
        Symbol,
        Operator,
    }

    let mut tokens: Vec<Token> = Vec::new();
    let mut character_stack: Vec<char> = Vec::new();
    let mut expect = Expected::Symbol;
    for c in expression.chars() {
        match expect {
            Expected::Symbol => {
                if c == '(' {
                    tokens.push(Token::ExpressionStart);
                    expect = Expected::Operator;
                } else if c == ')' {
                    if !character_stack.is_empty() {
                        let value: String = character_stack.iter().collect();
                        tokens.push(Token::Symbol(
                            value.parse().expect("Symbol has to be an Integer"),
                        ));
                        character_stack.clear();
                    }
                    tokens.push(Token::ExpressionEnd)
                } else if c == ' ' {
                    if !character_stack.is_empty() {
                        let value: String = character_stack.iter().collect();
                        tokens.push(Token::Symbol(
                            value.parse().expect("Symbol has to be an Integer"),
                        ));
                        character_stack.clear();
                    }
                } else {
                    character_stack.push(c);
                }
            }
            Expected::Operator => {
                if c != ' ' && c != '(' && c != ')' {
                    character_stack.push(c);
                } else if c == ' ' && !character_stack.is_empty() {
                    let value: String = character_stack.iter().collect();
                    tokens.push(Token::Function(value));
                    character_stack.clear();
                    expect = Expected::Symbol;
                } else if c == '(' && !character_stack.is_empty() {
                    let value: String = character_stack.iter().collect();
                    tokens.push(Token::Function(value));
                    character_stack.clear();
                    tokens.push(Token::ExpressionStart);
                } else if c == ')' && !character_stack.is_empty() {
                    let value: String = character_stack.iter().collect();
                    tokens.push(Token::Function(value));
                    character_stack.clear();
                    expect = Expected::Symbol;
                    tokens.push(Token::ExpressionEnd);
                }
            }
        }
    }
    Ok(tokens)
}
