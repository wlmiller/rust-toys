use std::collections::HashMap;
use std::f64::consts as consts;
use std::rc::Rc as Rc;

use interpreter;
use interpreter::Interpreter as Interpreter;
use interpreter::Value as Value;
use interpreter::EvalError as EvalError;
use parser::Node as Node;

#[derive(Clone)]
pub struct Lambda {
    pub params: Vec<Node>,
    pub body: Node,
    pub name: String
}

impl Lambda {
    pub fn new(params: Vec<Node>, body: Node) -> Lambda {
        Lambda { params: params, body: body, name: "aaa".to_string() }
    }
}

#[derive(Clone)]
pub struct Environment {
    pub env: HashMap<String, Value>,
    pub outer: Option<Box<Environment>>
}

impl Environment {
    pub fn new(outer: Option<Box<Environment>>) -> Environment {
        let mut env = Environment { env: HashMap::new(), outer: outer };
        
        env.initialize();

        env
    }
    
    pub fn new_empty(outer: Option<Box<Environment>>) -> Environment {
        Environment { env: HashMap::new(), outer: outer }
    }
    
    pub fn initialize(&mut self) {
        let mut env = HashMap::new();
        env.insert("begin".to_string(),  Value::Function(Rc::new(begin)));
        env.insert("+".to_string(),      Value::Function(Rc::new(add)));
        env.insert("-".to_string(),      Value::Function(Rc::new(sub)));
        env.insert("*".to_string(),      Value::Function(Rc::new(mul)));
        env.insert("/".to_string(),      Value::Function(Rc::new(div)));
        env.insert("pow".to_string(),    Value::Function(Rc::new(pow)));
        env.insert("define".to_string(), Value::Function(Rc::new(def)));
        env.insert("set!".to_string(),   Value::Function(Rc::new(def)));
        env.insert(">".to_string(),      Value::Function(Rc::new(gt)));
        env.insert(">=".to_string(),     Value::Function(Rc::new(gte)));
        env.insert("<".to_string(),      Value::Function(Rc::new(lt)));
        env.insert("<=".to_string(),     Value::Function(Rc::new(lte)));
        env.insert("=".to_string(),      Value::Function(Rc::new(eq)));
        env.insert("equal?".to_string(), Value::Function(Rc::new(eq)));
        env.insert("not".to_string(),    Value::Function(Rc::new(not)));
        env.insert("list".to_string(),   Value::Function(Rc::new(list)));
        env.insert("car".to_string(),    Value::Function(Rc::new(car)));
        env.insert("cdr".to_string(),    Value::Function(Rc::new(cdr)));
        env.insert("cons".to_string(),   Value::Function(Rc::new(cons)));
        env.insert("empty?".to_string(), Value::Function(Rc::new(emptyq)));
        env.insert("if".to_string(),     Value::Function(Rc::new(if_fn)));
        env.insert("map".to_string(),    Value::Function(Rc::new(map)));
        env.insert("sin".to_string(),    Value::Function(Rc::new(sin)));
        env.insert("cos".to_string(),    Value::Function(Rc::new(cos)));
        env.insert("tan".to_string(),    Value::Function(Rc::new(tan)));
        env.insert("asin".to_string(),   Value::Function(Rc::new(asin)));
        env.insert("acos".to_string(),   Value::Function(Rc::new(acos)));
        env.insert("atan".to_string(),   Value::Function(Rc::new(atan)));
        env.insert("exp".to_string(),    Value::Function(Rc::new(exp)));
        env.insert("log".to_string(),    Value::Function(Rc::new(log)));
        env.insert("log10".to_string(),  Value::Function(Rc::new(log10)));
        env.insert("pi".to_string(),     Value::Float(consts::PI));
        env.insert("e".to_string(),      Value::Float(consts::E));
        
        self.env = env;
    }
    
    pub fn get(&self, label: &String) -> Option<&Value> {
        match self.env.get(label) {
            Some(val) => Some(val),
            None => match self.outer {
                Some(ref outer) => outer.get(label),
                None => None
            }
        }
    }
    
    pub fn set(&mut self, label: String, value: Value) {
        self.env.insert(label, value);
    }
}

fn begin(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    let mut val: Result<Value, EvalError> = Ok(Value::Void);
    
    for node in xs {
        val = interpreter.eval_node(node);
    }
    
    val
}

fn add(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 2 {
        return Err(EvalError { message: "'+' takes exactly two arguments".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => match val {
            Value::Bool(true)  => Value::Int(1),
            Value::Bool(false) => Value::Int(0),
            val                => val
        },
        err      => return err
    };
    
    let y = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => match val {
            Value::Bool(true)  => Value::Int(1),
            Value::Bool(false) => Value::Int(0),
            val                => val
        },
        err      => return err
    };

    match (x, y) {
        (Value::Int(x), Value::Int(y))     => Ok(Value::Int(x + y)),
        (Value::Float(x), Value::Int(y))   => Ok(Value::Float(x + y as f64)),
        (Value::Int(x), Value::Float(y))   => Ok(Value::Float(x as f64 + y)),
        (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x + y)),
        (Value::Symbol(val), _) | (_, Value::Symbol(val)) => Err(EvalError { message: format!("Unknown symbol {}", val).to_string() }),
        _                                  => Err(EvalError { message: "Invalid types for '+'".to_string() })
    }
}

fn sub(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 2 {
        return Err(EvalError { message: "'' takes exactly two arguments".to_string() })
    }

    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => match val {
            Value::Bool(true)  => Value::Int(1),
            Value::Bool(false) => Value::Int(0),
            val                => val
        },
        err      => return err
    };
    
    let y = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => match val {
            Value::Bool(true)  => Value::Int(1),
            Value::Bool(false) => Value::Int(0),
            val                => val
        },
        err      => return err
    };

    match (x, y) {
        (Value::Int(x), Value::Int(y))     => Ok(Value::Int(x - y)),
        (Value::Float(x), Value::Int(y))   => Ok(Value::Float(x - y as f64)),
        (Value::Int(x), Value::Float(y))   => Ok(Value::Float(x as f64 - y)),
        (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x - y)),
        (Value::Symbol(val), _) | (_, Value::Symbol(val)) => Err(EvalError { message: format!("Unknown symbol {}", val).to_string() }),
        _                                  => Err(EvalError { message: "Invalid types for '-'".to_string() })
    }
}

fn mul(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 2 {
        return Err(EvalError { message: "'*' takes exactly two arguments".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => match val {
            Value::Bool(true)  => Value::Int(1),
            Value::Bool(false) => Value::Int(0),
            val                => val
        },
        err      => return err
    };
    
    let y = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => match val {
            Value::Bool(true)  => Value::Int(1),
            Value::Bool(false) => Value::Int(0),
            val                => val
        },
        err      => return err
    };
    
    match (x, y) {
        (Value::Int(x), Value::Int(y))     => Ok(Value::Int(x * y)),
        (Value::Float(x), Value::Int(y))   => Ok(Value::Float(x * y as f64)),
        (Value::Int(x), Value::Float(y))   => Ok(Value::Float(x as f64 * y)),
        (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x * y)),
        (Value::Symbol(val), _) | (_, Value::Symbol(val)) => Err(EvalError { message: format!("Unknown symbol {}", val).to_string() }),
        _                                  => Err(EvalError { message: "Invalid types for '*'".to_string() })
    }
}

fn div(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 2 {
        return Err(EvalError { message: "'/' takes exactly two arguments".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => match val {
            Value::Bool(true)  => Value::Int(1),
            Value::Bool(false) => Value::Int(0),
            val                => val
        },
        err      => return err
    };
    
    let y = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => match val {
            Value::Bool(true)  => Value::Int(1),
            Value::Bool(false) => Value::Int(0),
            val                => val
        },
        err      => return err
    };
    
    match (x, y) {
        (_, Value::Int(0)) | (_, Value::Float(0.0)) => Err(EvalError { message: "Invalid division by zero".to_string() }),
        (Value::Int(x), Value::Int(y))     => Ok(Value::Int(x / y)),
        (Value::Float(x), Value::Int(y))   => Ok(Value::Float(x / y as f64)),
        (Value::Int(x), Value::Float(y))   => Ok(Value::Float(x as f64 / y)),
        (Value::Float(x), Value::Float(y)) => Ok(Value::Float(x / y)),
        (Value::Symbol(val), _) | (_, Value::Symbol(val)) => Err(EvalError { message: format!("Unknown symbol {}", val).to_string() }),
        _                                  => Err(EvalError { message: "Invalid types for '/'".to_string() })
    }
}

fn pow(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 2 {
        return Err(EvalError { message: "'pow' takes exactly two arguments".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    let y = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match (x, y) {
        (Value::Int(x), Value::Int(y))           => Ok(Value::Float((x as f64).powi(y))),
        (Value::Float(x), Value::Int(y))         => Ok(Value::Float(x.powi(y))),
        (Value::Int(x), Value::Float(y))         => Ok(Value::Float((x as f64).powf(y))),
        (Value::Float(x), Value::Float(y))       => Ok(Value::Float(x.powf(y))),
        (Value::Symbol(val), _) | (_, Value::Symbol(val)) => Err(EvalError { message: format!("Unknown symbol {}", val).to_string() }),
        _                                        => Err(EvalError { message: "Invalid types for 'pow'".to_string() })
    }
}

pub fn def(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 2 {
        return Err(EvalError { message: "'define' takes exactly two arguments".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    let y = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => val,
        err     => return err
    };

    match (x, y) {
        (Value::Symbol(label), val) => { interpreter.env.set(label, val); Ok(Value::Void) },
        _ => Err(EvalError { message: format!("Can't define {}", xs[0]).to_string() })
    }
}

fn gt(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 2 {
        return Err(EvalError { message: "'>' takes exactly two arguments".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    
    let y = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match (x, y) {
        (Value::Int(x), Value::Int(y))     => Ok(Value::Bool(x > y)),
        (Value::Float(x), Value::Int(y))   => Ok(Value::Bool(x > y as f64)),
        (Value::Int(x), Value::Float(y))   => Ok(Value::Bool(x as f64 > y)),
        (Value::Float(x), Value::Float(y)) => Ok(Value::Bool(x > y)),
        (Value::Symbol(val), _) | (_, Value::Symbol(val)) => Err(EvalError { message: format!("Unknown symbol {}", val).to_string() }),
        _                                  => Err(EvalError { message: "Invalid types for '>'".to_string() })
    }
}

fn gte(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 2 {
        return Err(EvalError { message: "'>=' takes exactly two arguments".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    
    let y = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match (x, y) {
        (Value::Int(x), Value::Int(y))     => Ok(Value::Bool(x >= y)),
        (Value::Float(x), Value::Int(y))   => Ok(Value::Bool(x >= y as f64)),
        (Value::Int(x), Value::Float(y))   => Ok(Value::Bool(x as f64 >= y)),
        (Value::Float(x), Value::Float(y)) => Ok(Value::Bool(x >= y)),
        (Value::Symbol(val), _) | (_, Value::Symbol(val)) => Err(EvalError { message: format!("Unknown symbol {}", val).to_string() }),
        _                                  => Err(EvalError { message: "Invalid types for '>='".to_string() })
    }
}

fn lt(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 2 {
        return Err(EvalError { message: "'<' takes exactly two arguments".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    
    let y = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match (x, y) {
        (Value::Int(x), Value::Int(y))     => Ok(Value::Bool(x < y)),
        (Value::Float(x), Value::Int(y))   => Ok(Value::Bool(x < y as f64)),
        (Value::Int(x), Value::Float(y))   => Ok(Value::Bool((x as f64) < y)),
        (Value::Float(x), Value::Float(y)) => Ok(Value::Bool(x < y)),
        (Value::Symbol(val), _) | (_, Value::Symbol(val)) => Err(EvalError { message: format!("Unknown symbol {}", val).to_string() }),
        _                                  => Err(EvalError { message: "Invalid types for '<'".to_string() })
    }
}

fn lte(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 2 {
        return Err(EvalError { message: "'<=' takes exactly two arguments".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    
    let y = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match (x, y) {
        (Value::Int(x), Value::Int(y))     => Ok(Value::Bool(x <= y)),
        (Value::Float(x), Value::Int(y))   => Ok(Value::Bool(x <= y as f64)),
        (Value::Int(x), Value::Float(y))   => Ok(Value::Bool(x as f64 <= y)),
        (Value::Float(x), Value::Float(y)) => Ok(Value::Bool(x <= y)),
        (Value::Symbol(val), _) | (_, Value::Symbol(val)) => Err(EvalError { message: format!("Unknown symbol {}", val).to_string() }),
        _                                  => Err(EvalError { message: "Invalid types for '<='".to_string() })
    }
}

fn eq(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 2 {
        return Err(EvalError { message: "'=' takes exactly two arguments".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    
    let y = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match (x, y) {
        (Value::Int(x), Value::Int(y))         => Ok(Value::Bool(x == y)),
        (Value::Float(x), Value::Int(y))       => Ok(Value::Bool(x == y as f64)),
        (Value::Int(x), Value::Float(y))       => Ok(Value::Bool(x as f64 == y)),
        (Value::Float(x), Value::Float(y))     => Ok(Value::Bool(x == y)),
        (Value::Literal(x), Value::Literal(y)) => Ok(Value::Bool(x == y)),
        (Value::Symbol(val), _) | (_, Value::Symbol(val)) => Err(EvalError { message: format!("Unknown symbol {}", val).to_string() }),
        _                                  => Err(EvalError { message: "Invalid types for '='".to_string() })
    }
}

fn not(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'not' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
       Value::Bool(val) => Ok(Value::Bool(!val)),
       _ => Err(EvalError { message: "Invalid type for 'not'".to_string() })
    }
}

fn list(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    let mut vals: Vec<Value> = Vec::new();
    
    for node in xs {
        vals.push(match interpreter.eval_node(node) {
            Ok(val)  => val,
            err      => return err
        });
    }

    Ok(Value::List(vals))
}

fn emptyq(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'empty?' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
        Value::List(vals) => Ok(Value::Bool(vals.len() == 0)),
        _                 => Err(EvalError { message: "Invalid type for 'empty?'".to_string() })
    }
}

fn car(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'car' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
        Value::List(vals) => Ok(vals[0].clone()),
        _                 => Err(EvalError { message: "Invalid type for 'car'".to_string() })
    }
}

fn cdr(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'cdr' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
        Value::List(vals) => {
            let mut vals = vals.clone();
            vals.remove(0);
            Ok(Value::List(vals))
        },
        _                 => Err(EvalError { message: "Invalid type for 'cdr'".to_string() })
    }
}

fn cons(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 2 {
        return Err(EvalError { message: "'cons' takes exactly two arguments".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    let ys = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match (x, ys) {
        (v, Value::List(vals)) => {
            let mut vals = vals.clone();
            vals.insert(0, v);
            Ok(Value::List(vals))
        },
        _ => Err(EvalError { message: "Invalid type for 'cons'".to_string() })
    }
}

fn if_fn(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 3 {
        return Err(EvalError { message: "'if' takes exactly three arguments".to_string() })
    }
    
    let test: bool = match interpreter.eval_node(xs[0].clone()) {
        Ok(Value::Bool(val)) => val,
        _                    => return Err(EvalError { message: "'if' requires a boolean test".to_string() })
    };
    if test {
        interpreter.eval_node(xs[1].clone())
    } else {
        interpreter.eval_node(xs[2].clone())
    }
}

fn map(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError>  {
    if xs.len() != 2 {
        return Err(EvalError { message: "'map' takes exactly two arguments".to_string() })
    }
    
    let func = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    let list = match interpreter.eval_node(xs[1].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match (func, list) {
        (Value::Function(func), Value::List(vals)) => {
            let res_slice = vals.iter().map(|i| func(interpreter, vec![interpreter::convert_to_node(i.clone())]));
            let mut res: Vec<Value> = Vec::new();
            for r in res_slice {
                match r {
                    Ok(val) => res.push(val),
                    _ => ()
                }
            }
            Ok(Value::List(res))
        },
        (Value::Lambda(ref lambda), Value::List(ref vals)) => {
            let mut res: Vec<Value> = Vec::new();

            for val in vals.clone() {
                match interpreter.eval_lambda(lambda.clone(), vec![xs[0].clone(), interpreter::convert_to_node(val)]) {
                    Ok(val) => res.push(val),
                    err => return err 
                }
            }
            Ok(Value::List(res))
        },
        _                 => Err(EvalError { message: "Invalid type for 'map'".to_string() })
    }
}

// I'd like to combine all of the following (and many sets of the above) into a single function.
// I tried making a general fn(fn(f64) -> f64) -> fn(_), but it didn't work.
// Unfortunately, rust doesn't allow a function to capture external state, so there's no way
// to get the specific function I want to call into the returned function.  You can do it
// with a closure, but closures and fn pointers are not interchangeable.
fn sin(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'sin' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
        Value::Int(int)     => Ok(Value::Float((int as f64).sin())),
        Value::Float(float) => Ok(Value::Float(float.sin())),
        _                   => Err(EvalError { message: "Invalid type for 'sin'".to_string() })
    }
}

fn cos(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'cos' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
        Value::Int(int)     => Ok(Value::Float((int as f64).cos())),
        Value::Float(float) => Ok(Value::Float(float.cos())),
        _                   => Err(EvalError { message: "Invalid type for 'cos'".to_string() })
    }
}

fn tan(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'tan' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
        Value::Int(int)     => Ok(Value::Float((int as f64).tan())),
        Value::Float(float) => Ok(Value::Float(float.tan())),
        _                   => Err(EvalError { message: "Invalid type for 'tan'".to_string() })
    }
}

fn asin(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'asin' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
        Value::Int(int)     => Ok(Value::Float((int as f64).asin())),
        Value::Float(float) => Ok(Value::Float(float.asin())),
        _                   => Err(EvalError { message: "Invalid type for 'asin'".to_string() })
    }
}

fn acos(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'acos' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
        Value::Int(int)     => Ok(Value::Float((int as f64).acos())),
        Value::Float(float) => Ok(Value::Float(float.acos())),
        _                   => Err(EvalError { message: "Invalid type for 'acos'".to_string() })
    }
}

fn atan(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'atan' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
        Value::Int(int)     => Ok(Value::Float((int as f64).atan())),
        Value::Float(float) => Ok(Value::Float(float.atan())),
        _                   => Err(EvalError { message: "Invalid type for 'atan'".to_string() })
    }
}

fn exp(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'exp' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
        Value::Int(int)     => Ok(Value::Float((int as f64).exp())),
        Value::Float(float) => Ok(Value::Float(float.exp())),
        _                   => Err(EvalError { message: "Invalid type for 'exp'".to_string() })
    }
}

fn log(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'log' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
        Value::Int(int)     => Ok(Value::Float((int as f64).ln())),
        Value::Float(float) => Ok(Value::Float(float.ln())),
        _                   => Err(EvalError { message: "Invalid type for 'log'".to_string() })
    }
}

fn log10(interpreter: &mut Interpreter, xs: Vec<Node>) -> Result<Value, EvalError> {
    if xs.len() != 1 {
        return Err(EvalError { message: "'log10' takes exactly one argument".to_string() })
    }
    
    let x = match interpreter.eval_node(xs[0].clone()) {
        Ok(val) => val,
        err     => return err
    };
    match x {
        Value::Int(int)     => Ok(Value::Float((int as f64).log10())),
        Value::Float(float) => Ok(Value::Float(float.log10())),
        _                   => Err(EvalError { message: "Invalid type for 'log10'".to_string() })
    }
}