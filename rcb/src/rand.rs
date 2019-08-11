use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub fn run() {
    run_f64();
    run_i32();
    rand_dis();
}
pub fn run_f64() {
    let mut rng = rand::thread_rng();
    println!("Random f64: {}", rng.gen::<f64>());
}

pub fn run_i32() {
    let mut rng = rand::thread_rng();
    println!("Random i32: {}", rng.gen::<i32>());
}

fn rand_dis() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);
    loop {
        let throw = die.sample(&mut rng);
        println!("Rool the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}
