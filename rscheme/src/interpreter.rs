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
    Function(&'static str, Rc<fn(&mut Interpreter, Vec<Node>) -> Result<Value, EvalError>>),
    Lambda(Lambda),
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
                for p in lambda.clone().params {
                    params_str = format!("{}{}{}", params_str, sep, p);
                    sep = " ".to_string();
                }
                write!(f, "(lambda ({}) ({}))", params_str, lambda.body)
            },
            Value::Function(name, _) => write!(f, "{}", name),
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
        self.eval_node(tree.clone())
    }
    
    pub fn eval_node(&mut self, node: Node) -> Result<Value, EvalError> {
        match node {
            Node::ValueWrapper(val) => Ok(*val),
            Node::Int(val)    => Ok(Value::Int(val)),
            Node::Float(val)  => Ok(Value::Float(val)),
            Node::Bool(val)   => Ok(Value::Bool(val)),
            Node::Symbol(val) => {
                match self.env.get(&val) {
                    Some(res) => Ok(res.clone()),
                    None => Ok(Value::Symbol(val))
                }
            }
            Node::List(nodes)  => {
                let func_result = self.eval_node(nodes[0].clone());
                match func_result {
                    Ok(func_val) => {
                        match func_val {
                            Value::Symbol(val) => {
                                if val == "quote".to_string() {
                                    Ok(Interpreter::quote_node(nodes[1].clone()))
                                } else if val == "lambda".to_string() {
                                    let params: Vec<Node> = match nodes[1].clone() {
                                        Node::List(nodes) => nodes,
                                        _                 => Vec::new()
                                    };
                                    let body = nodes[2].clone();
                                    let lambda = Lambda::new(params, body);
                                    Ok(Value::Lambda(lambda))
                                } else {
                                    Err(EvalError { message: format!("Unknown function {}", val).to_string() })
                                }
                            },
                            Value::Function(_, func) => {
                                let mut args: Vec<Node> = nodes.clone();
                                args.remove(0);
                                match func(self, args) {
                                    Ok(val) => Ok(val),
                                    Err(err) => Err(err)
                                 }
                            },
                            Value::Lambda(lambda) => {
                                self.eval_lambda(lambda, nodes)
                            },
                            _ => {
                                Ok(func_val.clone())
                            }
                        }
                    },
                    Err(err) => Err(err)
                }
            }
        }
    }
    
    pub fn eval_lambda(&mut self, lambda: Lambda, nodes: Vec<Node>) -> Result<Value, EvalError> {
        let mut env = Environment::new_empty(Some(Box::new(self.env.clone())));
        let params = lambda.params;
        let body = lambda.body;

        if nodes.len() - 1 != params.len() {
            return Err(EvalError { message: format!("{} expects {} params, got {}", nodes[0], params.len(), nodes.len() - 1).to_string() })
        }

        for i in 0..params.len() {
            match params[i] {
                Node::Symbol(ref val) => {
                    match self.eval_node(nodes[i + 1].clone()) {
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
       
        interpreter.eval(body)
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
                for i in 0..params.len() {
                    match params[i].clone() {
                        Node::Symbol(param) => {
                            if label == param {
                                return values[i].clone()
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
    
    fn quote_node(node: Node) -> Value {
        match node {
            Node::Int(int)          => Value::Literal(int.to_string()),
            Node::Float(float)      => Value::Literal(float.to_string()),
            Node::Symbol(ref value) => Value::Literal(value.clone()),
            Node::Bool(true)        => Value::Literal("#t".to_string()),
            Node::Bool(false)       => Value::Literal("#f".to_string()),
            Node::List(nodes)       => {
                let mut vals: Vec<Value> = Vec::new();
                for node in nodes {
                    vals.push(Interpreter::quote_node(node.clone()));
                }
                Value::List(vals)
            },
            _                       => Value::Void
        }
    }
}