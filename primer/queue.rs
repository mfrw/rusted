pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn new() -> Box<Queue> {
        Box::new(Queue {
            older: Vec::new(),
            younger: Vec::new(),
        })
    }

    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

pub fn main() {
    let mut q = Queue::new();
    q.push('a');
    q.push('b');
    q.push('c');
    q.push('d');
    println!("{}", q.pop().unwrap());
}
