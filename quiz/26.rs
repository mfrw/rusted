fn main() {
    let input = vec![1, 2, 3];

    let pairity = input.iter().map(|x| {
        print!("{}", x);
        x % 2
    });

    for p in pairity {
        print!("{}", p);
    }
    println!("");
}
