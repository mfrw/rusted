pub fn run() {
    sort();
}

fn sort() {
    let mut vec = vec![1, 5, 1, 3, 17, 6, 5];
    vec.sort();
    println!("{:?}", vec);
}
