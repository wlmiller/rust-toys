#![cfg(test)]
use interpreter::*;
use parser;

fn run_test(source: &str, interpreter: &mut Interpreter) -> Result<Value, EvalError> {
    let tree = parser::parse(parser::tokenize(source.to_string()));
    match tree {
        Ok(val)  => interpreter.eval(val),
        Err(err) => Err(EvalError{ message: err.message })
    }
}

#[test]
fn test_artihmetic() {
let mut interpreter = Interpreter::new();
    
    if let Ok(Value::Int(4)) = run_test("(+ 2 2)", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Int(210)) = run_test("(+ (* 2 100) (* 1 10))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
}

#[test]
fn test_if() {
    let mut interpreter = Interpreter::new();
    
    if let Ok(Value::Int(2)) = run_test("(if (> 6 5) (+ 1 1) (+ 2 2))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Int(4)) = run_test("(if (< 6 5) (+ 1 1) (+ 2 2))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
}

#[test]
fn test_def() {
    let mut interpreter = Interpreter::new();
    
    if let Ok(Value::Void) = run_test("(define x 3)", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Int(3)) = run_test("x", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Int(6)) = run_test("(+ x x)", &mut interpreter) {
    } else {
        panic!("Failed");
    }
}

#[test]
fn test_begin() {
    let mut interpreter = Interpreter::new();
    
    if let Ok(Value::Int(3)) = run_test("(begin (define x 1) (set! x (+ x 1)) (+ x 1))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
}

#[test]
fn test_lambda() {
    let mut interpreter = Interpreter::new();
    
    if let Ok(Value::Int(10)) = run_test("((lambda (x) (+ x x)) 5)", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Void) = run_test("(define twice (lambda (x) (* 2 x)))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Int(10)) = run_test("(twice 5)", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Void) = run_test("(define compose (lambda (f g) (lambda (x) (f (g x)))))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::List(vals)) = run_test("((compose list twice) 5)", &mut interpreter) {
        if vals.len() != 1 {
            panic!("Failed");
        }
        
        if let Value::Int(10) = vals[0] {
        } else {
            panic!("Failed")
        }
    } else {
        panic!("Failed");
    }

    if let Ok(Value::Void) = run_test("(define repeat (lambda (f) (compose f f)))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Int(20)) = run_test("((repeat twice) 5)", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Int(80)) = run_test("((repeat (repeat twice)) 5)", &mut interpreter) {
    } else {
        panic!("Failed");
    }
}

#[test]
fn test_fact() {
    let mut interpreter = Interpreter::new();
    
    if let Ok(Value::Void) = run_test("(define fact (lambda (n) (if (<= n 1) 1 (* n (fact (- n 1))))))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Int(6)) = run_test("(fact 3)", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    // (fact 50) will cause an overflow for i32
    if let Ok(Value::Int(479001600)) = run_test("(fact 12)", &mut interpreter) {
    } else {
        panic!("Failed");
    }
}

#[test]
fn test_abs() {
    let mut interpreter = Interpreter::new();
    
    if let Ok(Value::Void) = run_test("(define abs (lambda (n) ((if (> n 0) + -) 0 n)))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(val) = run_test("(list (abs -3) (abs 0) (abs 3))", &mut interpreter) {
        assert_eq!(format!("{}", val), "(3 0 3)");
    } else {
        panic!("Failed");
    }
}

#[test]
fn test_combine() {
    let mut interpreter = Interpreter::new();
    
    if let Ok(Value::Void) = run_test("(define combine (lambda (f) \
        (lambda (x y) \
        (if (null? x) (quote ()) \
            (f (list (car x) (car y)) \
                ((combine f) (cdr x) (cdr y)))))))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Void) = run_test("(define zip (combine cons))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(val) = run_test("(zip (list 1 2 3 4) (list 5 6 7 8))", &mut interpreter) {
        assert_eq!(format!("{}", val), "((1 5) (2 6) (3 7) (4 8))");
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Void) = run_test("(define riff-shuffle (lambda (deck) (begin \
        (define take (lambda (n seq) (if (<= n 0) (quote ()) (cons (car seq) (take (- n 1) (cdr seq)))))) \
        (define drop (lambda (n seq) (if (<= n 0) seq (drop (- n 1) (cdr seq))))) \
        (define mid (lambda (seq) (/ (length seq) 2))) \
        ((combine append) (take (mid deck) deck) (drop (mid deck) deck)))))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(val) = run_test("(riff-shuffle (list 1 2 3 4 5 6 7 8))", &mut interpreter) {
        assert_eq!(format!("{}", val), "(1 5 2 6 3 7 4 8)");
    } else {
        panic!("Failed");
    }
    
    if let Ok(val) = run_test("(riff-shuffle (list 1 2 3 4 5 6 7 8))", &mut interpreter) {
        assert_eq!(format!("{}", val), "(1 5 2 6 3 7 4 8)");
    } else {
        panic!("Failed");
    }
    
    // These are a repeat of tests above, needed for the following riff-shuffle test
    if let Ok(Value::Void) = run_test("(define compose (lambda (f g) (lambda (x) (f (g x)))))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(Value::Void) = run_test("(define repeat (lambda (f) (compose f f)))", &mut interpreter) {
    } else {
        panic!("Failed");
    }
    
    if let Ok(val) = run_test("((repeat riff-shuffle) (list 1 2 3 4 5 6 7 8))", &mut interpreter) {
        assert_eq!(format!("{}", val), "(1 3 5 7 2 4 6 8)");
    } else {
        panic!("Failed");
    }
    
    if let Ok(val) = run_test("(riff-shuffle (riff-shuffle (riff-shuffle (list 1 2 3 4 5 6 7 8))))", &mut interpreter) {
        assert_eq!(format!("{}", val), "(1 2 3 4 5 6 7 8)");
    } else {
        panic!("Failed");
    }
}