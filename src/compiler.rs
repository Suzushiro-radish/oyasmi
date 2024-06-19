use crate::tokenizer;

pub fn compile(input: String) -> String {
    let mut tokens = tokenizer::tokenize(&input).into_iter();

    let mut output = String::new();

    output.push_str("(module\n");
    output.push_str("  (export \"_start\" (func $main))\n");
    output.push_str("  (func $main\n");
    output.push_str("    (result i32)\n");

    while let Some(token) = tokens.next() {
        match token {
            tokenizer::Token::Add => {
                if let Some(tokenizer::Token::Int(i)) = tokens.next() {
                    output.push_str(&format!("    i32.const {}\n", i));
                } else {
                    panic!("Expected int after add");
                }
                output.push_str("    i32.add\n");
            }
            tokenizer::Token::Sub => {
                if let Some(tokenizer::Token::Int(i)) = tokens.next() {
                    output.push_str(&format!("    i32.const {}\n", i));
                } else {
                    panic!("Expected int after sub");
                }
                output.push_str("    i32.sub\n");
            }
            tokenizer::Token::Int(i) => {
                output.push_str(&format!("    i32.const {}\n", i));
            }
        }
    }

    output.push_str("    return\n");
    output.push_str("  )\n)\n");

    output
}
