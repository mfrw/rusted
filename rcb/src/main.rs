mod bit;
mod crypto;
mod datetime;
mod par;
mod rand;
mod sort;
mod tar;
mod term;

fn main() {
    rand::run();
    sort::run();
    term::run();
    tar::run();
    par::run();
    crypto::run();
    bit::run();
    datetime::run();
}
