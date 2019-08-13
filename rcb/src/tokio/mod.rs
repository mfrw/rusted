mod echo;
mod hello;

pub fn run() {
    println!("Going to tokio...");
    hello::hello_world();
    echo::run();
}
