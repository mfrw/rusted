use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug)]
struct Node<T> {
    cont: RefCell<Peekable<IntoIter<T>>>,
}

impl<T> Node<T> {
    fn new(v: Vec<T>) -> Self {
        Self {
            cont: RefCell::new(v.into_iter().peekable()),
        }
    }

    fn from(pi: Peekable<IntoIter<T>>) -> Self {
        Self {
            cont: RefCell::new(pi),
        }
    }
}

impl<T: Ord + Eq> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut it_a = self.cont.borrow_mut();
        let mut it_b = other.cont.borrow_mut();
        it_b.peek().cmp(&it_a.peek())
    }
}

impl<T> PartialOrd for Node<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut it_a = self.cont.borrow_mut();
        let mut it_b = other.cont.borrow_mut();
        it_b.peek().partial_cmp(&it_a.peek())
    }
}

impl<T> Eq for Node<T>
where
    T: Ord,
{
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T> PartialEq for Node<T>
where
    T: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        let mut it_a = self.cont.borrow_mut();
        let mut it_b = other.cont.borrow_mut();
        it_b.peek().eq(&it_a.peek())
    }
}

#[derive(Debug)]
struct KthLargest<T> {
    pq: BinaryHeap<Node<T>>,
}

impl<T> Iterator for KthLargest<T>
where
    T: Ord + Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.pq.is_empty() {
            let v = self.pq.pop().unwrap();
            let mut it = v.cont.into_inner();
            if let Some(elt) = it.next() {
                self.pq.push(Node::from(it));
                return Some(elt);
            }
        }
        None
    }
}

fn kth_largest<T>(vs: Vec<Vec<T>>, k: usize) -> Option<T>
where
    T: Ord + Debug,
{
    let pq = BinaryHeap::from_iter(vs.into_iter().map(|v| Node::new(v)));
    let kl = KthLargest { pq };
    kl.into_iter().nth(k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(
            Some(92),
            kth_largest(
                vec![
                    vec![4, 5, 6, 7],
                    vec![91, 92, 93],
                    vec![0, 0, 0],
                    vec![],
                    vec![2, 2, 3],
                ],
                11
            )
        );
    }
}
