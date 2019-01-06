use std::env;

pub fn main() {
    vars();
    cwd();
}

fn vars() {
    for (k, v) in env::vars() {
        println!("{} => {}", k, v);
    }
}

fn cwd() -> std::io::Result<()> {
    let p = env::current_dir()?;
    println!("{}", p.display());
    Ok(())
}
