use std::error::Error;
use std::io;
use std::process;

pub fn csv_writer() {
    //let dollar_films = vec![
    //("A Fistful of Dollars", "Rojo", 1964),
    //("For a Few Dollars More", "El Indio", 1965),
    //("The Good, the Bad and the Ugly", "Tuco", 1966),
    //];
    //let _path = "westerns.cvs";

    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&["a", "b", "c"]).expect("CSV Error");
}

pub fn csv_reader() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = "test.csv";
    let mut rdr = csv::Reader::from_path(path)?;
    {
        let headers = rdr.headers()?;
        println!("{:?}", headers);
    }
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
