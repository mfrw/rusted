use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use tar::Archive;

pub fn run() {
    if let Err(err) = untar() {
        println!("{}", err);
    }
    if let Err(err) = tar_dir() {
        println!("{}", err);
    }
}

fn untar() -> Result<(), std::io::Error> {
    let path = "test.tar.gz";
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

fn tar_dir() -> Result<(), std::io::Error> {
    let tar_gz = File::create("src.tgz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("./", "./src")?;
    Ok(())
}
