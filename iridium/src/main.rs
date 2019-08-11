pub mod instruction;
pub mod vm;

fn main() {
    println!("Hello, world!");
    let v = vm::VM::new();
    println!("{:?}", v);
}
