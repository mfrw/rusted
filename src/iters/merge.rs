use std::cmp::Ordering;
use std::iter::Peekable;

pub trait Merger: Iterator {
    fn merge<T>(self, other: T) -> Merge<Self, T::IntoIter>
    where
        Self: Sized,
        T: IntoIterator<Item = Self::Item>,
    {
        merge(self, other.into_iter())
    }
}

impl<T> Merger for T where T: Iterator + Sized {}

fn merge<L, R>(left: L, right: R) -> Merge<L, R>
where
    L: IntoIterator<Item = R::Item>,
    R: IntoIterator,
{
    Merge::new(left, right)
}

pub struct Merge<L, R>
where
    L: IntoIterator<Item = R::Item>,
    R: IntoIterator,
{
    left: Peekable<L::IntoIter>,
    right: Peekable<R::IntoIter>,
}

impl<L, R> Merge<L, R>
where
    L: IntoIterator<Item = R::Item>,
    R: IntoIterator,
{
    pub fn new(left: L, right: R) -> Self {
        Self {
            left: left.into_iter().peekable(),
            right: right.into_iter().peekable(),
        }
    }
}

impl<L, R> Iterator for Merge<L, R>
where
    L: IntoIterator<Item = R::Item>,
    R: IntoIterator,
    L::Item: Ord,
{
    type Item = L::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let which = match (self.left.peek(), self.right.peek()) {
            (Some(l), Some(r)) => Some(l.cmp(r)),
            (Some(_), None) => Some(Ordering::Less),
            (None, Some(_)) => Some(Ordering::Greater),
            (None, None) => None,
        };
        match which {
            Some(Ordering::Less) => self.left.next(),
            Some(Ordering::Equal) => self.left.next(),
            Some(Ordering::Greater) => self.right.next(),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sorted_slices_test() {
        let v1 = [1, 2, 3, 4];
        let v2 = [1, 2, 3, 4];
        let res = merge(v1, v2).collect::<Vec<_>>();
        assert_eq!(vec![1, 1, 2, 2, 3, 3, 4, 4], res);
    }

    #[test]
    fn vec_test() {
        let v1 = [1, 2, 3, 4];
        let v2 = [1, 2, 3, 4];
        let v3 = [1, 2, 3, 4];
        let v4 = [1, 2, 3, 4];
        let v5 = v1
            .into_iter()
            .merge(v2)
            .merge(v3)
            .merge(v4)
            .collect::<Vec<_>>();
        assert_eq!(v5.last(), Some(&4));
    }
}
