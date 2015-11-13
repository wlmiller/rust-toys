use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io;
use std::env;

mod parser;
mod environment;
mod interpreter;

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
    
     let tree = parser::parse(parser::tokenize(source));
     match tree {
        Ok(val) => { 
            let res = Interpreter::new().eval(val);
            match res {
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

#[cfg(test)]
mod tests {
    use interpreter::*;
    use parser;
    
    fn run_test(source: String, interpreter: &mut Interpreter) -> Result<Value, EvalError> {
        let tree = parser::parse(parser::tokenize(source));
        match tree {
            Ok(val)  => interpreter.eval(val),
            Err(err) => Err(EvalError{ message: err.message })
        }
    }
    
    #[test]
    fn test_circle_area() {
        let source = "(begin (define circle-area (lambda (r) (* pi (* r r)))) (circle-area 3))".to_string();
        match run_test(source, &mut Interpreter::new()) {
            Ok(val) => match val {
                Value::Float(float) => assert!(((float - 28.274333877)/28.274333877).abs() < 0.001),
                _                   => panic!("circle-area should result in a float")
            },
            Err(err) => panic!(err.message)
        }    
    }
    
    #[test]
    fn test_fact() {
        let mut interpreter = Interpreter::new();
        
        match run_test("(define fact (lambda (n) (if (<= n 1) 1 (* n (fact (- n 1))))))".to_string(), &mut interpreter) {
            Ok(val) => match val {
                Value::Void => (),
                _           => panic!("define should return Void")
            },
            Err(err) => panic!(err.message)
        }
        
        match run_test("(fact 10)".to_string(), &mut interpreter) {
            Ok(val) => match val {
                Value::Int(int) => assert_eq!(int, 3628800),
                _               => panic!("(fact 10) should result in an int")
            },
            Err(err) => panic!(err.message)
        }
    }
    
    #[test]
    fn test_count() {
        let mut interpreter = Interpreter::new();
        
        let _res = run_test("(begin (define first car) (define rest cdr))".to_string(), &mut interpreter);
        let _res = run_test("(define count (lambda (item L) (if (not (empty? L)) (+ (equal? item (first L)) (count item (rest L))) 0)))".to_string(), &mut interpreter);
        
        match run_test("(count 0 (list 0 1 2 3 0 0))".to_string(), &mut interpreter) {
            Ok(val) => match val {
                Value::Int(int) => assert_eq!(int, 3),
                _               => panic!("count should result in an int")
            },
            Err(err) => panic!(err.message)
        }
        
        match run_test("(count (quote the) (quote (the more the merrier the bigger the better)))".to_string(), &mut interpreter) {
            Ok(val) => match val {
                Value::Int(int) => assert_eq!(int, 4),
                _               => panic!("count should result in an int")
            },
            Err(err) => panic!(err.message)
        }
    }
    
    #[test]
    fn test_repeat_twice() {
        let mut interpreter = Interpreter::new();

        let _res = run_test("(define twice (lambda (x) (* 2 x)))".to_string(), &mut interpreter);
        
        match run_test("(twice 5)".to_string(), &mut interpreter) {
            Ok(val) => match val {
                Value::Int(int) => assert_eq!(int, 10),
                _               => panic!("should result in an int")
            },
            Err(err) => panic!(err.message)
        }
        
        let _res = run_test("(define repeat (lambda (f) (lambda (x) (f (f x)))))".to_string(), &mut interpreter);
        
        match run_test("((repeat twice) 10)".to_string(), &mut interpreter) {
            Ok(val) => match val {
                Value::Int(int) => assert_eq!(int, 40),
                _               => panic!("should result in an int")
            },
            Err(err) => panic!(err.message)
        }
        
        match run_test("((repeat twice) 10)".to_string(), &mut interpreter) {
            Ok(val) => match val {
                Value::Int(int) => assert_eq!(int, 40),
                _               => panic!("should result in an int")
            },
            Err(err) => panic!(err.message)
        }
        
        match run_test("((repeat (repeat twice)) 10)".to_string(), &mut interpreter) {
            Ok(val) => match val {
                Value::Int(int) => assert_eq!(int, 160),
                _               => panic!("should result in an int")
            },
            Err(err) => panic!(err.message)
        }
        
        match run_test("((repeat (repeat (repeat twice))) 10)".to_string(), &mut interpreter) {
            Ok(val) => match val {
                Value::Int(int) => assert_eq!(int, 2560),
                _               => panic!("count should result in an int")
            },
            Err(err) => panic!(err.message)
        }
    }
    
    #[test]
    fn test_pow() {
        let source = "(pow 2 16)".to_string();
        match run_test(source, &mut Interpreter::new()) {
            Ok(val) => match val {
                Value::Float(float) => assert!(((float - 65536.0)/65535.0).abs() < 0.001),
                _                   => panic!("pow should result in a float")
            },
            Err(err) => panic!(err.message)
        }    
    }
    
    #[test]
    fn test_fib() {
        let mut interpreter = Interpreter::new();

        let _res = run_test("(define fib (lambda (n) (if (< n 2) 1 (+ (fib (- n 1)) (fib (- n 2))))))".to_string(), &mut interpreter);
        let _res = run_test("(define range (lambda (a b) (if (= a b) (quote ()) (cons a (range (+ a 1) b)))))".to_string(), &mut interpreter);

        match run_test("(range 0 10)".to_string(), &mut interpreter) {
            Ok(val) => match val {
                Value::List(list) => assert_eq!(list.iter().map(|x| match *x { 
                    Value::Int(int) => int,
                    _               => -1
                }).collect::<Vec<i32>>(), (0..10).collect::<Vec<i32>>()),
                _                 => panic!("should result in a list")
            },
            Err(err) => panic!(err.message)
        }
        
        match run_test("(map fib (range 0 10))".to_string(), &mut interpreter) {
            Ok(val) => match val {
                Value::List(list) => assert_eq!(list.iter().map(|x| match *x { 
                    Value::Int(int) => int,
                    _               => -1
                }).collect::<Vec<i32>>(), vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]),
                _                 => panic!("should result in a list")
            },
            Err(err) => panic!(err.message)
        }
    }
}