use std::{env, string::ParseError};
use oyasmi::compiler;

fn main() -> Result<(), ParseError> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }

    let input = args[1].clone();
    let output = compiler::compile(input).unwrap();
    println!("{}", output);
    Ok(())
}
