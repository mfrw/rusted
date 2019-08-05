use std::io;

pub fn csv_writer() {
    //let dollar_films = vec![
    //("A Fistful of Dollars", "Rojo", 1964),
    //("For a Few Dollars More", "El Indio", 1965),
    //("The Good, the Bad and the Ugly", "Tuco", 1966),
    //];

    let _path = "westerns.cvs";
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(&["a", "b", "c"]).expect("CSV Error");
}

pub fn csv_reader() {
    let path = "test.csv";
    let mut rdr = csv::Reader::from_path(path).expect("Not Found");
    for result in rdr.records() {
        let record = result.expect("CSV error");
        println!("{:?}", record);
    }
}
