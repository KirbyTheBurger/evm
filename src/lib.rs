use std::mem;

use crate::values::Value;

pub mod values;

#[derive(Debug)]
pub enum IR {
    Push(Value),
    Pop,
}

pub struct VM {
    input: Vec<IR>,
    stack: Vec<Value>,
}

impl VM {
    pub fn new(input: Vec<IR>) -> VM {
        VM {
            input,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        for i in mem::take(&mut self.input) {
            self.execute(i);
        }
    }

    fn execute(&mut self, instruction: IR) {
        println!("current instruction: {:?}", instruction);
        match instruction {
            IR::Push(v) => self.push(v),
            IR::Pop => { self.pop(); },
        }
    }

    fn push(&mut self, v: Value) {
        println!("Pushed value {:?} to stack", v);
        self.stack.push(v);
    }

    fn pop(&mut self) -> Value {
        let v = match self.stack.pop() {
            Some(v) => {
                println!("Popped value {:?} from stack", v);
                v
            },
            None => {
                println!("Attempted to pop from stack while empty, defaulting to `None`");
                Value::None
            },
        };
        v
    }
}