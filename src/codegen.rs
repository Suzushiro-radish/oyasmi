use crate::ast::{self, Node, Statement};

pub fn codegen(statements: Vec<Statement>) -> String {
    let mut output = String::new();
    let mut statements = statements.into_iter().peekable();

    output.push_str("(module\n");
    output.push_str("  (export \"_start\" (func $main))\n");
    output.push_str("  (func $main\n");
    output.push_str("    (result i32)\n");

    while let Some(statement) = statements.next() {
        codegen_statement(statement, &mut output);

        if statements.peek().is_some() {
            output.push_str("    drop\n");
        }
    }

    output.push_str("  )\n)\n");

    output
}

fn codegen_statement(stmt: ast::Statement, output: &mut String) {
    codegen_node(stmt.node, output)
}

fn codegen_node(ast: Node, output: &mut String) {
    match ast {
        Node::Number(n) => {
            output.push_str(&format!("    i32.const {}\n", n));
        }
        Node::Add(lhs, rhs) => {
            codegen_node(*lhs, output);
            codegen_node(*rhs, output);
            output.push_str("    i32.add\n");
        }
        Node::Sub(lhs, rhs) => {
            codegen_node(*lhs, output);
            codegen_node(*rhs, output);
            output.push_str("    i32.sub\n");
        }
        Node::Mul(lhs, rhs) => {
            codegen_node(*lhs, output);
            codegen_node(*rhs, output);
            output.push_str("    i32.mul\n");
        }
        Node::Div(lhs, rhs) => {
            codegen_node(*lhs, output);
            codegen_node(*rhs, output);
            output.push_str("    i32.div_s\n");
        }
    }
}
