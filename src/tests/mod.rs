use crate::{eval, functions::setup_functions};

#[test]
fn eval_add() {
    let functions = setup_functions();
    assert_eq!(eval("(+ 2 2)", &functions).unwrap(), 4.);
}

#[test]
fn eval_sub() {
    let functions = setup_functions();
    assert_eq!(eval("(- 4 2)", &functions).unwrap(), 2.);
}
#[test]
fn eval_mul() {
    let functions = setup_functions();
    assert_eq!(eval("(* 3 3)", &functions).unwrap(), 9.);
}
#[test]
fn eval_square() {
    let functions = setup_functions();
    assert_eq!(eval("(square 9)", &functions).unwrap(), 81.);
}

#[test]
fn eval_nested() {
    let functions = setup_functions();
    assert_eq!(eval("(square (+ 3 3 (- 4 2)))", &functions).unwrap(), 64.);
}
