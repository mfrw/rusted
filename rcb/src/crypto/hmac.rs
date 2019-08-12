use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::{hmac, rand};

pub fn run() -> Result<(), Unspecified> {
    let rng = rand::SystemRandom::new();
    let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng)?;
    let message = "Legitimate and important message";
    let signature = hmac::sign(&key, message.as_bytes());
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;
    println!("HMAC: {}", HEXUPPER.encode(signature.as_ref()));
    Ok(())
}
