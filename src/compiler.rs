use crate::{tokenizer, parser, codegen};

pub fn compile(input: String) -> String {
    let mut tokens = tokenizer::tokenize(&input).into_iter().peekable();


    let node = parser::parse(&mut tokens);

    codegen::codegen(node)
}
