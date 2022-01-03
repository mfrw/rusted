use super::Stack;

pub struct IntoIter<T> {
    it: std::vec::IntoIter<T>,
}

impl<T> IntoIter<T> {
    pub fn new(stk: Stack<T>) -> Self {
        Self {
            it: stk.base.into_iter(),
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next_back()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_stk_test() {
        let stk = Stack {
            base: vec![1, 2, 3],
        };
        let mut iter = stk.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
