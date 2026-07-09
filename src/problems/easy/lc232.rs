#![allow(dead_code)]
struct MyQueue {
    stack: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
    }

    fn pop(&mut self) -> i32 {
        let mut temp_stack = vec![];
        while self.stack.is_empty() == false {
            temp_stack.push(self.stack.pop().unwrap());
        }
        let res_value = temp_stack.pop().unwrap();
        while temp_stack.is_empty() == false {
            self.stack.push(temp_stack.pop().unwrap());
        }
        res_value
    }

    fn peek(&self) -> i32 {
        *self.stack.first().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack.len() == 0
    }
}
