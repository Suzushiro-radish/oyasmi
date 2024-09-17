use std::collections::HashSet;

use crate::ast::{self, Node, Statement};

pub fn codegen(statements: Vec<Statement>) -> String {
    let mut statements = statements.into_iter().peekable();

    let mut vars = HashSet::new();

    let mut signature = String::new();

    signature.push_str("(module\n");
    signature.push_str("  (export \"_start\" (func $main))\n");
    signature.push_str("  (func $main\n");
    signature.push_str("    (result i32)\n");

    let mut body = String::new();

    while let Some(statement) = statements.next() {
        codegen_statement(&statement, &mut body, &mut vars);

        if statements.peek().is_some() && match statement {
            Statement::Node(_) => true,
            Statement::Assign(_, _) => false,
        } {
            body.push_str("    drop\n");
        }
    }

    let var_declarations = vars.iter().fold(String::new(), |mut acc, v| {
        use std::fmt::Write; // write!マクロを使うために必要
        writeln!(&mut acc, "    (local ${} i32)", v).unwrap();
        acc
    });

    let trailing_parenthesis = String::from("  )\n)\n");

    let output = format!(
        "{}{}{}{}",
        signature, var_declarations, body, trailing_parenthesis
    );

    output
}

fn codegen_statement(stmt: &ast::Statement, output: &mut String, vars: &mut HashSet<String>) {
    match stmt {
        Statement::Node(node) => {
            codegen_node(node, output);
        }
        Statement::Assign(name, node) => {
            codegen_assign(name.as_str(), node, output, vars);
        }
    }
}

fn codegen_assign(name: &str, node: &Node, output: &mut String, vars: &mut HashSet<String>) {
    codegen_node(node, output);
    vars.insert(name.to_string());
    output.push_str(&format!("    local.set ${}\n", name));
}

fn codegen_node(ast: &Node, output: &mut String) {
    match ast {
        Node::Variable(name) => {
            output.push_str(&format!("    local.get ${}\n", name));
        }
        Node::Number(n) => {
            output.push_str(&format!("    i32.const {}\n", n));
        }
        Node::Add(lhs, rhs) => {
            codegen_node(lhs, output);
            codegen_node(rhs, output);
            output.push_str("    i32.add\n");
        }
        Node::Sub(lhs, rhs) => {
            codegen_node(lhs, output);
            codegen_node(rhs, output);
            output.push_str("    i32.sub\n");
        }
        Node::Mul(lhs, rhs) => {
            codegen_node(lhs, output);
            codegen_node(rhs, output);
            output.push_str("    i32.mul\n");
        }
        Node::Div(lhs, rhs) => {
            codegen_node(lhs, output);
            codegen_node(rhs, output);
            output.push_str("    i32.div_s\n");
        }
    }
}
