use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, std::io::Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok(context.finish())
}
pub fn run() -> Result<(), std::io::Error> {
    let path = "src.tgz";
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;
    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));
    Ok(())
}
