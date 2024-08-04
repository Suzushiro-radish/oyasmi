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
        Node::Expression(expr) => {
            codegen_node(*expr.lhs, output);
            codegen_node(*expr.rhs, output);

            match expr.op {
                ast::Operator::Add => {
                    output.push_str("    i32.add\n");
                }
                ast::Operator::Sub => {
                    output.push_str("    i32.sub\n");
                }
                ast::Operator::Mul => {
                    output.push_str("    i32.mul\n");
                }
                ast::Operator::Div => {
                    output.push_str("    i32.div_s\n");
                }
            }
        }
    }
}
