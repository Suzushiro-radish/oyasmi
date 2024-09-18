use crate::{codegen, parser::{self, ParseError}, tokenizer};

pub fn compile(input: String) -> Result<String, ParseError> {
    let mut tokens = tokenizer::tokenize(&input).into_iter().peekable();

    let node = parser::parse(&mut tokens)?;

    Ok(codegen::codegen(node))
}
