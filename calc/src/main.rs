use calc::{perform_operation, Operation};
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

fn parse_arguments(args: Vec<String>) -> Result<Operation<i32>, String> {
    match &args[..] {
        [_, oper, param1, param2] => str_to_int(&param1)
            .and_then(|x| str_to_int(&param2).map(|y| (x, y)))
            .and_then(|(x, y)| Operation::build(&oper, x, y)),
        _ => Err("Invalid arguments passed!".to_string()),
    }
}

fn str_to_int(s: &str) -> Result<i32, String> {
    s.parse()
        .map_err(|_| "Failed to parse input value as integer".to_string())
}
