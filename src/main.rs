use std::env;

mod compiler;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }

    let input = args[1].parse::<i32>().unwrap();

    print!("{}", compiler::compile(input));
}
