use crossbeam;

pub fn run() {
    let arr = &[1, 25, -4, 10];
    let max = find_max(arr);
    assert_eq!(max, Some(25));
    println!("Max is {}", max.unwrap());
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;
    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let min_l = thread_l.join().unwrap()?;
        let min_r = thread_r.join().unwrap()?;
        Some(min_l.max(min_r))
    })
    .unwrap()
}
