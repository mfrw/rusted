use super::Stack;

/*
pub struct IntoIter<I> {
    it: I,
}

impl<I: DoubleEndedIterator> IntoIter<I> {
    pub fn new(it: I) -> Self {
        Self { it }
    }
}

impl<I> Iterator for IntoIter<I>
where
    I: DoubleEndedIterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next_back()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;

    type IntoIter = IntoIter<std::vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.base.into_iter())
    }
}
*/

impl<T> Iterator for Stack<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.base.pop()
    }
}

impl<T> Stack<T> {
    pub fn iter(&self) -> core::slice::Iter<T> {
        self.base.iter()
    }
}

/*
pub struct Iter<'a, T: 'a> {
    it: std::slice::Iter<'a, T>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next_back()
    }
}
*/

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
