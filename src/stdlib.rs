use std::{cell::RefCell, rc::Rc};
use crate::interpreter::{Node, Environment};

pub fn define_var(symbol: &Node, value: &Node, env: Rc<RefCell<Environment>>) -> Node {
    if let Node::Symbol(string) = symbol {
        env.borrow_mut().data.insert(string.clone(), value.clone());
    }
    // dbg!(env);
    Node::Null
}

pub fn define_func(symbol: &Node, params: &Node, body: &Node, env: Rc<RefCell<Environment>>) -> Node {
    let mut symbol_name: String = String::new();
    let mut param_names: Vec<String> = vec![];

    if let Node::Symbol(string) = symbol {
        symbol_name = string.clone();
    }

    if let Node::List(nodes) = params {
        for node in nodes {
            if let Node::Symbol(string) = node {
                param_names.push(string.clone());
            }
        }
    }

    if let Node::List(_) = body {
        env.borrow_mut().data.insert(symbol_name, Node::Function(param_names, Box::new(body.clone())));
    }

    dbg!(env);

    Node::Null
}

pub fn do_arithmetic_operation(nodes: &Vec<Node>, operator: &String) -> Node {
    let mut res = 0.0;
    if let Node::Float(float) = &nodes[0] {
        res = *float;
    }

    for node in nodes.iter().skip(1) {
        if let Node::Float(float) = node {
            match operator.as_str() {
                "+" => res += float,
                "*" => res *= float,
                "-" => res -= float,
                "/" => res /= float,
                _ => {

                }
            }
        }
    }

    Node::Float(res)
}
