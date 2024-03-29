#![allow(dead_code)]

mod from;
mod iter;

#[derive(Debug, Clone)]
pub struct Stack<T> {
    base: Vec<T>,
}

impl<T> std::ops::Deref for Stack<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T> std::ops::DerefMut for Stack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
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
    #[test]
    fn iter_test() {
        let stk: Stack<_> = vec![1, 2, 3, 4, 5].into();
        assert_eq!(stk.iter().count(), 5);
    }
}
