pub fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(3);
    let a = v.pop().unwrap();
    println!("{}", a);
}
