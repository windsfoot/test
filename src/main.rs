use std::io;
use csv::Reader;
use std::error::Error;
fn main() {
    RdrFromFile().unwrap();
}

fn RdrFromFile() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("操作.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}