use std::cmp::Ordering;

fn main() {
    let mut v = vec![
        "hello".to_string(),
        "a".to_string(),
        "z".to_string(),
        "world".to_string(),
        "this".to_string(),
    ];
    sort_names(&mut v);

    println!("{:?}", v);
}

fn sort_names(v: &mut Vec<String>) {
    fn cmp_name(a: &String, b: &String) -> Ordering {
        if a < b {
            return Ordering::Less;
        }
        Ordering::Greater
    }
    v.sort_by(cmp_name);
}
