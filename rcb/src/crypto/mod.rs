mod hmac;
mod sha256;

pub fn run() {
    if let Err(err) = sha256::run() {
        println!("Error in SHA-256: {}", err);
    }

    if let Err(err) = hmac::run() {
        println!("Error in HMAC: {}", err);
    }
}
