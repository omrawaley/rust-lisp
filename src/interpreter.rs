use std::{cell::RefCell, collections::HashMap, rc::Rc};
use crate::stdlib;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Symbol(String),
    Float(f64),
    LParen,
    RParen,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Null,
    Symbol(String),
    Float(f64),
    List(Vec<Node>),
    Function(Vec<String>, Box<Node>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    pub data: HashMap<String, Node>,
}

impl Environment {
    fn get(&self, key: &String) -> Option<Node> {
        if let Some(value) = self.data.get(key) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(key)
        } else {
            None
        }
    }
}

pub struct Interpreter {
    pub src: String,
    tokens: Vec<Token>,
    ast: Node,
    env: Rc<RefCell<Environment>>,
    pub res: Node,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter {
            src: String::new(),
            tokens: vec![],
            ast: Node::Null,
            env: Rc::new(RefCell::new(Environment { parent: None, data: HashMap::new() })),
            res: Node::Null,
        };
        interpreter.create_default_environment();
        interpreter
    }

    pub fn reset(&mut self) {
        self.src = String::new();
        self.tokens = vec![];
        self.ast = Node::Null;
    }

    fn create_default_environment(&mut self) {
        
    }

    pub fn tokenize(&mut self) {
        let expanded_src = self.src.replace("(", " ( ").replace(")", " ) ");
        let words = expanded_src.split_whitespace();

        for word in words {
            match word {
                "(" => self.tokens.push(Token::LParen),
                ")" => self.tokens.push(Token::RParen),
                _ => {
                    let token = word.parse::<f64>();
                    match token {
                        Ok(_) => self.tokens.push(Token::Float(token.unwrap())),
                        Err(_) => self.tokens.push(Token::Symbol(word.to_string())),
                    }
                }
            }
        }
    }

    pub fn parse(&mut self) {
        let ast = self.parse_list(&mut self.tokens.clone()).first().unwrap().clone();

        match ast {
            Node::List(_) => self.ast = ast,
            _ => {
                eprintln!("ERROR::PARSER::EXPECTED_LIST");
            }
        }

        // dbg!(&self.ast);
    }

    fn parse_list(&mut self, tokens: &mut Vec<Token>) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];

        while !tokens.is_empty() {
            let token = tokens.remove(0);

            match token {
                Token::Symbol(string) => nodes.push(Node::Symbol(string)),
                Token::Float(float) => nodes.push(Node::Float(float)),
                Token::LParen => {
                    let list = Node::List(self.parse_list(tokens));
                    nodes.push(list);
                }
                Token::RParen => {
                    return nodes
                }
            }
        }

        nodes
    }

    pub fn evaluate(&mut self) {
        self.res = self.evaluate_node(&self.ast, self.env.clone());
        // dbg!(&self.res);
    }

    fn evaluate_node(&self, node: &Node, env: Rc<RefCell<Environment>>) -> Node {
        match node {
            Node::Null => Node::Null,
            Node::Symbol(string) => self.evaluate_symbol(string, env),
            Node::Float(float) => Node::Float(*float),
            Node::List(nodes) => self.evaluate_list(nodes, env),
            // Node::Function(func) => self.evaluate_function(func),
            // Node::Function(param_names, body) => self.evaluate_function(param_names, &body, env)
            Node::Function(_, _) => Node::Null
        }
    }

    fn evaluate_nodes(&self, nodes: &Vec<Node>, env: Rc<RefCell<Environment>>) -> Vec<Node> {
        nodes
            .iter()
            .map(|node| self.evaluate_node(node, env.clone()))
            .collect()
    }

    fn evaluate_symbol(&self, symbol: &String, env: Rc<RefCell<Environment>>) -> Node {
        let entry = env.borrow().get(symbol);
        match entry {
            Some(node) => node,
            None => {
                eprintln!("ERROR::EVALUATOR::UNIDENTIFIED_SYMBOL");
                Node::Null
            }
        }
    }

    fn evaluate_list(&self, nodes: &Vec<Node>, env: Rc<RefCell<Environment>>) -> Node {
        let evaluated_nodes = &self.evaluate_nodes(&nodes, env.clone());

        if let Node::Symbol(string) = &nodes[0] {
            match string.as_str() {
                "let" => stdlib::define_var(&nodes[1], &evaluated_nodes[2], env.clone()),
                "func" => stdlib::define_func(&nodes[1], &nodes[2], &nodes[3], env.clone()),
                "+" | "-" | "*" | "/" => stdlib::do_arithmetic_operation(&evaluated_nodes[1..].to_vec(), &string),
                _ => {
                    if env.borrow().data.contains_key(string) {
                        let node = env.borrow().data.get(string).unwrap().clone();
                        if let Node::Function(_, _) = node {
                            self.evaluate_function(&node, &evaluated_nodes[1], env.clone())
                        } else {
                            Node::List(evaluated_nodes.clone())
                        }
                    } else {
                        Node::Null
                    }
                }
            }
        } else {
            Node::List(evaluated_nodes.clone())
        }
    }

    fn evaluate_function(&self, function_node: &Node, param_list_node: &Node, env: Rc<RefCell<Environment>>) -> Node {
        dbg!(function_node);
        dbg!(param_list_node);

        let new_env: Rc<RefCell<Environment>> = Rc::new(RefCell::new(Environment {parent: Some(env), data: HashMap::new()}));

        if let Node::Function(param_names, body) = function_node {
            if let Node::List(nodes) = param_list_node {
                for (i, node) in nodes.iter().enumerate() {
                    new_env.borrow_mut().data.insert(param_names[i].clone(), node.clone());
                }
                
                dbg!(&new_env);
            }

            if let Node::List(nodes) = &**body {
                return self.evaluate_list(nodes, new_env)
            }
        }

        Node::Null
    }

}
