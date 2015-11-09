use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::vec::Vec;
use std::env;
use std::collections::BTreeMap;

struct Tape {
    pos: usize,
    tape: Vec<isize>
}

impl Tape {
    fn new() -> Tape {
        Tape { pos: 0, tape: vec![0] }
    }
    
    // Get the value at the current position
    fn get(&self) -> isize {
        self.tape[self.pos]
    }
    
    // Get the character value at the current position
    fn getc(&self) -> char {
        self.tape[self.pos] as u8 as char
    }
    
    // Set the current position to the given character
    fn setc(&mut self, char: char) {
        self.tape[self.pos] = char as isize;
    }
    
    // Increment the current position
    fn inc(&mut self) {
        self.tape[self.pos] += 1;
    }
    
    // Decrement the current position
    fn dec(&mut self) {
        self.tape[self.pos] -= 1;
    }
    
    // Advance the pointer
    fn adv(&mut self) {
        self.pos += 1;
        if self.tape.len() <= self.pos {
            self.tape.push(0);
        }
    }
    
    // Devance the pointer
    fn dev(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }
}

struct Program {
    code: Vec<char>,
    loop_map: BTreeMap<usize, usize>,
    stdin: Vec<char>
}

impl Program {
    fn new(source: String, stdin: String) -> Program {
        let mut code: Vec<char> = Vec::new();
        let mut loop_map = BTreeMap::new();
        let mut leftstack = Vec::new(); // A stack of positions of left brackets '['
        let mut pc = 0;
        
        for c in source.chars() {
            match c {
                '+' | '-' | '.' | ',' | '<' | '>' => (),
                '[' => { leftstack.push(pc); },
                ']' => match leftstack.pop() {
                    Some(left) => {
                        // left is the position of the corresponding left bracket, pc is the current position
                        loop_map.insert(left, pc);
                        loop_map.insert(pc, left);
                    }
                    None => {}
                },
                _ => { continue; }
            }
            code.push(c);
            pc += 1;
        }
        Program{ code: code, loop_map: loop_map, stdin: stdin.chars().collect() }
    }
    
    fn run(&self) {
        let mut pc: usize = 0;
        let mut spc: usize = 0;
        let len = self.code.len();
        let mut tape = Tape::new();
        
        while pc < len {
            match self.code[pc] {
                '+' => tape.inc(),
                '-' => tape.dec(),
                '>' => tape.adv(),
                '<' => tape.dev(),
                ',' => {
                    if spc < self.stdin.len() {
                        tape.setc(self.stdin[spc]);
                        spc += 1;
                    }
                },
                '[' => {
                    if tape.get() == 0 {
                        // Skip to the corresponding closing bracket
                        pc = self.loop_map[&pc];
                    }
                },
                ']' => {
                    if tape.get() != 0 {
                        // Go back to the corresponding opening bracket
                        pc = self.loop_map[&pc]
                    }
                },
                '.' => { print!("{}", tape.getc()); },
                _ => ()
            }
            pc += 1;
        }
    }
}

fn main() {
    let arg1 = env::args().nth(1).unwrap();
    let arg2 = env::args().nth(2);
    
    let path = Path::new(&arg1);
    let mut source = String::new();
    let mut file = File::open(&path).unwrap();
    file.read_to_string(&mut source).unwrap();
    
    let mut stdin: String = String::new();
    match arg2 {
        Some(s) => stdin = s,
        None => ()
    }
    
    Program::new(source, stdin).run();
}