pub fn main() {
    let text = "I see the eighenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    println!("{}, {} \n", head, tail);

    print_pandovan();
}

fn print_pandovan() {
    let mut pandovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = pandovan[i - 3] + pandovan[i - 2];
        pandovan.push(next);
    }
    println!("P(1..10) = {:?}", pandovan);
}

fn bad() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    let u = s;
}
