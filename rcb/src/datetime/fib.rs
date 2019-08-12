use std::time::Instant;

pub fn basic_elapsed() {
    let start = Instant::now();
    expensive_fn(40u32);
    let duration = start.elapsed();
    println!("Elapsed: {:?}", duration);
}

fn expensive_fn(n: u32) {
    let result = fib(n);
    println!("fib({}) => {}", n, result);
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        3 => 2,
        4 => 3,
        5 => 5,
        _ => fib(n - 1) + fib(n - 2),
    }
}
