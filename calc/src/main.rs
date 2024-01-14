use std::{env, process};

fn main() {
    let result = parse_arguments(env::args().collect())
        .and_then(perform_operation)
        .unwrap_or_else(|err| {
            eprintln!("Error: {err}");
            process::exit(1);
        });
    println!("Result is: {result}");
}

fn parse_arguments(args: Vec<String>) -> Result<Operation, String> {
    match &args[..] {
        [_, oper, param1, param2] => str_to_int(&param1)
            .and_then(|x| str_to_int(&param2).map(|y| (x, y)))
            .and_then(|(x, y)| Operation::build(&oper, x, y)),
        _ => Err("Invalid arguments passed!".to_string()),
    }
}

fn str_to_int(s: &str) -> Result<i64, String> {
    s.parse()
        .map_err(|_| "Failed to parse input value as integer".to_string())
}

enum Operation {
    Add(i64, i64),
    Sub(i64, i64),
    Mult(i64, i64),
    Div(i64, i64),
}

impl Operation {
    fn build(operation: &str, param1: i64, param2: i64) -> Result<Operation, String> {
        match operation.to_lowercase().as_str() {
            "add" => Ok(Operation::Add(param1, param2)),
            "sub" => Ok(Operation::Sub(param1, param2)),
            "div" => Ok(Operation::Div(param1, param2)),
            "mult" => Ok(Operation::Mult(param1, param2)),
            unknown => Err(format!("Unknown operation - {}", unknown)),
        }
    }
}

fn perform_operation(operation: Operation) -> Result<i64, String> {
    match operation {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Sub(a, b) => Ok(a - b),
        Operation::Mult(a, b) => Ok(a * b),
        Operation::Div(_, 0) => Err("Cannot divide by 0".to_string()),
        Operation::Div(a, b) => Ok(a / b),
    }
}
