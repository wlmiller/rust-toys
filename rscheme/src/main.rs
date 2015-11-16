use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io;
use std::env;

mod parser;
mod environment;
mod interpreter;
mod tests;

use interpreter::Interpreter as Interpreter;

fn main() {
    match env::args().nth(1) {
        Some(val) => run_script(val),
        None => repl()
    }    
}

fn run_script(file_name: String) {
    let path = Path::new(&file_name);
    let mut source = String::new();
    let mut file = File::open(&path).unwrap();
    file.read_to_string(&mut source).unwrap();
    
     match parser::parse(parser::tokenize(source)) {
        Ok(val) => { 
            match Interpreter::new().eval(val) {
                Ok(val) => { println!("{}", val); },
                Err(err) => { println!("{}", err); }
            }
        },
        Err(err) => { println!("{}", err); }
     }
}

fn repl() {
    let stdin = io::stdin();
    let mut interpreter = Interpreter::new();
    
    loop {
        print!("rscheme> ");
        io::stdout().flush().ok().expect("Could not flush stdout");
        let mut line = String::new();
        let _res = stdin.read_line(&mut line);
        match parser::parse(parser::tokenize(line)) {
            Ok(node) => {
                match interpreter.eval(node) {
                    Ok(val)  => match val {
                        interpreter::Value::Void => (),
                        _           => println!("{}", val)
                    },
                    Err(err) => println!("{}", err)
                }
             },
             Err(err) => println!("{}", err)
        }
    }
}

