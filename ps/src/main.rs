use std::mem;

fn main() {
    vars();
    operators();
    loops();
    for_loop();
    match_statement();
    functions();
    closures();
    traits();
}

fn match_statement() {
    let country_code = 77;
    let country = match country_code {
        44 => "UK",
        46 => "Swden",
        1...999 => "unknown",
        _ => "invalid",
    };
    println!("country_code = {} & country = {}", country_code, country);
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

fn print_value(x: i32) {
    println!("value = {}", x);
}

fn functions() {
    print_value(33);
    let mut z = 1;
    println!("z = {}", z);
    increase(&mut z);
    println!("z = {}", z);

    let a = 3;
    let b = 4;
    let prod = product(a, b);
    println!("prod = {}", prod);
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

fn increase(x: &mut i32) {
    *x += 1;
}

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx - dy * dy).sqrt()
    }
}

fn say_hello() {
    println!("Hello!!");
}

fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x: i32| -> i32 { x + 1 };

    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let plus_two = |x| {
        let mut z = x;
        z += 2;
        z
    };

    println!("{} + 2 = {}", 3, plus_two(3));
}

trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name: name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says Hello!", self.name);
    }
}

fn traits() {
    let h = Human { name: "John" };
    h.talk();
}
