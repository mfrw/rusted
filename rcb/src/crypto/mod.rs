mod hmac;
mod pbkdf2;
mod sha256;

pub fn run() {
    if let Err(err) = sha256::run() {
        println!("Error in SHA-256: {}", err);
    }

    if let Err(err) = hmac::run() {
        println!("Error in HMAC: {}", err);
    }
    if let Err(err) = pbkdf2::run() {
        println!("Error in PBKDF2: {}", err);
    }
}
