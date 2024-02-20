use csv;
use std::error::Error;

fn main() {
    if let Err(err) = read_csv("data.csv") {
        println!("error running example: {}", err);
    }
}

fn read_csv(path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record.iter().collect::<Vec<_>>());
    }
    Ok(())
}
