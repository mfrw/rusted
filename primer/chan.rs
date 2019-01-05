use std::sync::mpsc::channel;
use std::thread;

pub fn main() {
    let (tx, rx) = channel();

    for i in 1..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }

    for _ in 1..10 {
        let j = rx.recv().unwrap();
        assert!(0 <= j && j < 10);
        println!("{}", j);
    }
}
