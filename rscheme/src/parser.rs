extern crate regex;

use std::fmt;
use std::slice;
use self::regex::Regex as Regex;

use interpreter::Value as Value;

#[derive(Clone)]
pub enum Node {
    Symbol(String),
    List(Vec<Node>),
    Int(i32),
    Float(f64),
    Complex(f64, f64),
    Bool(bool),
    String(String),
    // ValueWrapper is for occasions when a value needs to be treated as a Node
    ValueWrapper(Box<Value>)
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            Node::Symbol(val)       => write!(f, "{}", val),
            Node::Int(val)          => write!(f, "{}", val),
            Node::Float(val)        => write!(f, "{}", val),
            Node::Complex(real, im) => write!(f, "{}+{}i", real, im),
            Node::Bool(true)        => write!(f, "#t"),
            Node::Bool(false)       => write!(f, "#f"),
            Node::String(ref val)   => write!(f, "\"{}\"", val.replace("\"","\\\"")),
            Node::List(vals)        => { 
                let mut output = String::new();
                let mut sep = String::new();
                for val in vals {
                    output = format!("{}{}{}",output,sep,val.clone());
                    sep = " ".to_string();
                }
                write!(f, "({})", output)
            },
            Node::ValueWrapper(ref val) => write!(f, "{}", val)
        }
    }
}

pub enum Token {
    OpenParen,
    CloseParen,
    String(String),
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

    let program_spread = program.replace("(", " ( ").replace(")", " ) ").replace("\"", " \" ").replace("\\ \" ","\\\"");
    let split_whitespace = program_spread.split_whitespace();
    
    let mut in_quote = false;
    let mut quote_level = 0;
    let mut in_string = false;
    let mut string: Vec<String> = Vec::new();
    for token in split_whitespace {
        if !in_string {
            if token == "(" {
                tokens.push(Token::OpenParen);
                if in_quote { quote_level += 1; }
            } else if token == ")" {
                tokens.push(Token::CloseParen); 
                if in_quote { 
                    quote_level -= 1; 
                    if quote_level == 0 {
                        tokens.push(Token::CloseParen);
                    }
                }
            } else if token == "'" {
                in_quote = true;
                tokens.push(Token::OpenParen);
                tokens.push(Token::NonParen("quote".to_string()));
            } else if token == "\"" {
                in_string = true;
                string = Vec::new();
            } else {
                tokens.push(Token::NonParen(token.to_string()));
            }
        } else {
            if token == "\"" {
                tokens.push(Token::String(string.join(" ")));
                string = Vec::new();
                in_string = false;
            } else {
                string.push(token.replace("\\\"","\""));
            }
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
                Token::String(ref val) => Ok(Some(Node::String(val.clone()))),
                Token::NonParen(ref val) => {
                    if val == "#t" {
                        Ok(Some(Node::Bool(true)))
                    } else if val == "#f" {
                        Ok(Some(Node::Bool(false)))
                    } else {
                        let re = Regex::new(r"^(\d*\.?\d*)([\+-]\d*\.?\d*)i$").unwrap();
                        if re.is_match(val) {
                            for cap in re.captures_iter(val) {
                                let real_part = cap.at(1).unwrap_or("0").parse::<f64>();
                                let imaginary_part = match cap.at(2).unwrap_or("1") {
                                    "-" => Ok(-1.0),
                                    "+" => Ok(1.0),
                                    val => val.parse::<f64>()
                                };
                                match (real_part, imaginary_part) {
                                    (Ok(real), Ok(im)) => return Ok(Some(Node::Complex(real, im))),
                                    _                  => return Err(ParseError { message: format!("Error parsing complex constant {}", val) })
                                }
                            }
                        }
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