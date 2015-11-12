use std::fmt;
use std::slice;

use interpreter::Value as Value;

#[derive(Clone)]
pub enum Node {
    Symbol(String),
    List(Vec<Node>),
    Int(i32),
    Float(f64),
    Bool(bool),
    // ValueWrapper is for occasions when a value needs to be treated as a Node
    ValueWrapper(Box<Value>)
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            Node::Symbol(val) => { write!(f, "{}", val) },
            Node::Int(val)    => { write!(f, "{}", val) },
            Node::Float(val)  => { write!(f, "{}", val)  },
            Node::Bool(val)   => { write!(f, "{}", val)  },
            _                 => { write!(f, "_") } 
        }
    }
}

pub enum Token {
    OpenParen,
    CloseParen,
    NonParen(String)
}

pub struct ParseError {
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}

pub fn tokenize(program: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let program_spread = program.replace("(", " ( ").replace(")", " ) ");
    let split_whitespace = program_spread.split_whitespace();
    
    for token in split_whitespace {
        if token == "(" {
            tokens.push(Token::OpenParen);
        } else if token == ")" {
            tokens.push(Token::CloseParen); 
        } else {
            tokens.push(Token::NonParen(token.to_string()));
        }
    }

    tokens
}

pub fn parse(tokens: Vec<Token>) -> Result<Node, ParseError> {
    match parse_nodes(&mut tokens.iter(), 0) {
        Ok(val) => {
            if val.len() > 1 {
                Err(ParseError { message: "Only one outer level permitted".to_string() })
            } else {
                Ok(val[0].clone())
            }
        },
        Err(err) => Err(err)
    }
}

fn parse_nodes(mut tokens: &mut slice::Iter<Token>, depth: u32) -> Result<Vec<Node>, ParseError> {
    let mut node_list = Vec::new();
    loop {
        match try!(parse_node(&mut tokens, depth)) {
            Some(node) => node_list.push(node),
            None       => return Ok(node_list)
        }
    }
}

fn parse_node(mut tokens: &mut slice::Iter<Token>, depth: u32) -> Result<Option<Node>, ParseError> {
    match tokens.next() {
        Some(token) => {
            match *token {
                Token::OpenParen        => {
                    let inner = try!(parse_nodes(&mut tokens, depth + 1));
                    Ok(Some(Node::List(inner)))
                },
                Token::CloseParen       => {
                    if depth > 0 {
                        Ok(None)
                    } else {
                        Err(ParseError { message: "Unexpected close paren".to_string() })
                    }
                },
                Token::NonParen(ref val) => {
                    if val == "#t" {
                        Ok(Some(Node::Bool(true)))
                    } else if val == "#f" {
                        Ok(Some(Node::Bool(false)))
                    } else {
                        match val.parse::<i32>() {
                            Ok(int_val) => Ok(Some(Node::Int(int_val))),
                            _ => match val.parse::<f64>() {
                                Ok(float_val) => Ok(Some(Node::Float(float_val))),
                                _ => Ok(Some(Node::Symbol(val.clone())))
                            }
                        }
                    }
                }
            }
        },
        None => {
            if depth == 0 {
                Ok(None)
            } else {
                Err(ParseError { message: "Unexected end of input".to_string() })
            }
        }
    }
}