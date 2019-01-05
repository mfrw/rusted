use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::thread;

pub fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(2);
    heap.push(1);
    heap.push(5);
    heap.push(10);
    heap.push(-1);
    heap.push(33);

    for x in heap.iter() {
        println!("{}", x);
    }
    println!("{:?}", heap);
    println!("{}", heap.peek().unwrap());

    let mut vdq = VecDeque::new();
    vdq.push_back(1);
    vdq.push_front(10);
    println!("Size of vdq {}", vdq.capacity());
    vdq.shrink_to_fit();
    println!("Size of vdq {}", vdq.capacity());

    threadfn();
}

fn threadfn() {
    let child = thread::spawn(move || -> i64 { fib(45) });
    fib(45);
    println!("{}", child.join().unwrap());
}

fn fib(n: i64) -> i64 {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}
