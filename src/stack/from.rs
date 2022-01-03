use super::Stack;

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            base: iter.into_iter().collect(),
        }
    }
}

impl<T> From<Vec<T>> for Stack<T> {
    fn from(v: Vec<T>) -> Self {
        Stack { base: v }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_iter_test() {
        let stk = vec!["a", "b", "c"].into_iter().collect::<Stack<_>>();
        assert_eq!(stk.len(), 3);
    }

    #[test]
    fn into_stk_test() {
        let mut stk: Stack<i32> = vec![1, 2, 3, 4].into();
        assert_eq!(stk.pop(), Some(4));
    }
}
