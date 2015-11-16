use std::fmt;
use std::rc::Rc as Rc;

use environment::Environment as Environment;
use environment::Lambda as Lambda;
use parser::Node as Node;

#[derive(Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
    Bool(bool),
    Symbol(String),
    Literal(String),
    List(Vec<Value>),
    Function(&'static str, Rc<fn(&mut Interpreter, &Vec<Node>) -> Result<Value, EvalError>>),
    Lambda(Lambda),
    NodeWrapper(Node),
    Void
}

pub fn convert_to_node(val: Value) -> Node {
    Node::ValueWrapper(Box::new(val))
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Int(val)          => write!(f, "{}", val),
            Value::Float(val)        => write!(f, "{}", val),
            Value::Bool(true)        => write!(f, "#t"),
            Value::Bool(false)       => write!(f, "#f"),
            Value::Symbol(ref val) | Value::Literal(ref val) => write!(f, "{}", val),
            Value::List(ref vals)    => {
                let mut output = String::new();
                let mut sep = String::new();
                for val in vals {
                    output = format!("{}{}{}",output,sep,val.clone());
                    sep = " ".to_string();
                }
                write!(f, "({})", output)
            }
            Value::Lambda(ref lambda) => {
                let mut params_str = String::new();
                let mut sep = String::new();
                for p in &lambda.params {
                    params_str = format!("{}{}{}", params_str, sep, p);
                    sep = " ".to_string();
                }
                write!(f, "(lambda ({}) ({}))", params_str, lambda.body)
            },
            Value::Function(name, _) => write!(f, "{}", name),
            Value::NodeWrapper(ref node) => write!(f, "{}", node),
            Value::Void            => write!(f, "()")
        }
    }
}

pub struct EvalError {
    pub message: String, 
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EvalError: {}", self.message)
    }
}

#[derive(Clone)]
pub struct Interpreter {
    pub env: Environment
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { env: Environment::new(None) }
    }
    
    pub fn new_with_env(env: Environment) -> Interpreter{
        Interpreter { env: env }
    }

    pub fn eval(&mut self, tree: Node) -> Result<Value, EvalError> {
        self.eval_node(&tree)
    }
    
    pub fn eval_node(&mut self, node: &Node) -> Result<Value, EvalError> {
        let mut node = node.clone();
        loop {
            match self.eval_node_wrapped(&node) {
                Ok(Value::NodeWrapper(node_cont)) => node = node_cont,
                val                               => return val
            }
        }
    }
    
    pub fn eval_node_wrapped(&mut self, node: &Node) -> Result<Value, EvalError> {
        match node {
            &Node::ValueWrapper(ref val)  => Ok((**val).clone()),
            &Node::Int(val)               => Ok(Value::Int(val)),
            &Node::Float(val)             => Ok(Value::Float(val)),
            &Node::Bool(val)              => Ok(Value::Bool(val)),
            &Node::Symbol(ref val)            => {
                match self.env.get(&val) {
                    Some(res) => Ok(res.clone()),
                    None => Ok(Value::Symbol(val.clone()))
                }
            }
            &Node::List(ref nodes)  => {
                let func_result = self.eval_node(&nodes[0]);
                match func_result {
                    Ok(func_val) => {
                        match func_val {
                            Value::Symbol(val) => Err(EvalError { message: format!("Unknown function {}", val).to_string() }),
                            Value::Function(_, func) => {
                                let args = nodes[1..].to_vec();
                                match func(self, &args) {
                                    Ok(val) => Ok(val),
                                    Err(err) => Err(err)
                                 }
                            },
                            Value::Lambda(lambda)    => self.eval_lambda(lambda, nodes),
                            Value::NodeWrapper(node) => {
                                let mut node_vec: Vec<Node> = nodes.clone();
                                node_vec[0] = node.clone();
                                self.eval_node(&Node::List(node_vec))
                            },
                            _ => Err(EvalError { message: "Invalid function call".to_string() })
                        }
                    },
                    Err(err) => Err(err)
                }
            }
        }
    }
    
    pub fn eval_lambda(&mut self, lambda: Lambda, nodes: &Vec<Node>) -> Result<Value, EvalError> {
        let mut env = Environment::new_empty(Some(Box::new(self.env.clone())));
        let params = lambda.params;
        let body = lambda.body;

        if nodes.len() - 1 != params.len() {
            return Err(EvalError { message: format!("{} expects {} params, got {}", nodes[0], params.len(), nodes.len() - 1).to_string() })
        }

        for (i, p) in params.iter().enumerate() {
            match *p {
                Node::Symbol(ref val) => {
                    match self.eval_node(&nodes[i + 1]) {
                        Ok(res) => env.set(val.clone(), res),
                        Err(err) => return Err(err)
                    }
                },
                _ => return Err(EvalError { message: format!("Invalid parameter {}", params[i]).to_string() })
            }
        }
        // Make a new interpreter, with the current interpreter as its outer scope
        let mut interpreter = Interpreter::new_with_env(env.clone());
        let body = interpreter.inline_lambda_nodes(body, params, &nodes[1..]);
        
        interpreter.eval_node_wrapped(&body)
    }

    fn inline_lambda_nodes(&mut self, node: Node, params: Vec<Node>, values: &[Node]) -> Node {
        match node.clone() {
            Node::List(nodes) => {
                let mut temp_list: Vec<Node> = Vec::new();
                for node in nodes {
                    temp_list.push(self.inline_lambda_nodes(node, params.clone(), values));
                }
                Node::List(temp_list)
            },
            Node::Symbol(label) => {
                for (i,p) in params.iter().enumerate() {
                    match *p {
                        Node::Symbol(ref param) => {
                            if label == *param {
                                if let Ok(val) = self.eval_node(&values[i]) {
                                    return Node::ValueWrapper(Box::new(val));
                                }
                            }
                        },
                        _ => ()
                    }
                }
                node
            },
            _ => node
        }
    }
}