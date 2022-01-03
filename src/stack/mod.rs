#![allow(dead_code)]

mod iter;

pub use iter::IntoIter;

pub struct Stack<T> {
    base: Vec<T>,
}

/// Stack Public Interface
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            base: Default::default(),
        }
    }
    pub fn with_capacity(n: usize) -> Self {
        Self {
            base: Vec::with_capacity(n),
        }
    }

    pub fn push(&mut self, value: T) {
        self.base.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.base.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.base.last()
    }

    pub fn len(&self) -> usize {
        self.base.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_stk_test() {
        let mut stk = Stack::with_capacity(8);
        for i in 1..9 {
            stk.push(i);
        }
        assert_eq!(stk.len(), 8);
    }
}
