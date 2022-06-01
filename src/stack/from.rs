use super::Stack;

impl<T> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            base: iter.into_iter().collect(),
        }
    }
}

impl<T> From<&[T]> for Stack<T>
where
    T: Copy,
{
    fn from(sl: &[T]) -> Self {
        Stack::from_iter(sl.into_iter().map(|v| *v))
    }
}

impl<T, const N: usize> From<[T; N]> for Stack<T>
where
    T: Copy,
{
    fn from(arr: [T; N]) -> Self {
        Self::from_iter(arr)
    }
}

impl<T> From<Vec<T>> for Stack<T> {
    fn from(v: Vec<T>) -> Self {
        Self { base: v }
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
    #[test]
    fn from_array_test() {
        let arr: [i32; 4] = [1, 2, 3, 4];
        let mut stk: Stack<_> = arr.into();
        assert_eq!(stk.pop(), Some(4));
    }
    #[test]
    fn from_slice_test() {
        let v = vec![1, 2, 3, 4, 5];
        let sl = v.as_slice();
        let mut stk: Stack<_> = sl.into();
        assert_eq!(Some(5), stk.pop());
    }
}
