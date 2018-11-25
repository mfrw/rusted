use std::mem;

fn main() {
    vars();
    operators();
    loops();
    for_loop();
}

fn for_loop() {
    let a = 10;
    let b = 20;
    for (pos, num) in (a..b).enumerate() {
        println!("pos = {}, num = {}", pos, num);
    }
}

fn loops() {
    println!("while");
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        println!("x = {}", x);
    }

    println!("loop");
    let mut y = 1;
    loop {
        y *= 2;
        if y > (1 << 10) {
            break;
        }
        println!("y = {}", y);
    }
}

fn vars() {
    println!("FN: Vars");
    println!("******************************");
    let a: u8 = 123;
    println!("a = {}", a);

    let mut b: u8 = 10;
    println!("b = {}", b);
    b = 17;
    println!("b = {}", b);

    let z: isize = 17;
    println!("z = {}, bits in z={}", z, mem::size_of_val(&z));

    let g = true;
    println!("g = {}, sizeof g = {} ", g, mem::size_of_val(&g));
    println!("------------------------------");
}

fn operators() {
    println!("FN: operators");
    println!("******************************");
    let mut a = 2 + 3 * 4;
    println!("a = {}", a);
    a = 3;
    println!("a = {}", a);
    let a_cubed = i32::pow(a, 3);
    println!("a_cubed = {}", a_cubed);

    let c = 1 | 2;
    println!("{}", c);

    let two_to_10 = 1 << 10;
    println!("{}", two_to_10);

    let pi_less4 = std::f64::consts::PI < 4.0;
    println!("pi_less4 = {}", pi_less4);

    println!("------------------------------");
}
