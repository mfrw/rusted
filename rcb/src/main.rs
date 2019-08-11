pub mod rand;
pub mod sort;
pub mod tar;
pub mod term;

fn main() {
    rand::run();
    sort::run();
    term::run();
    tar::run();
}
