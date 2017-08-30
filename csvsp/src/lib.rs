extern crate csv;

use std::error::Error;
use std::fmt;
use std::fs::File;

#[derive(Debug)]
pub struct CommonError {
    text: &'static str,
}

impl CommonError {
    fn new(text: &'static str) -> CommonError {
        CommonError { text }
    }
}

impl Error for CommonError {
    fn description(&self) -> &str {
        self.text
    }
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

pub fn run(args: Vec<String>) -> Result<(), Box<Error>>{
    //PITFALL 0: - move in for loop
    //for x in &args { 
    //    println!("{}", x);
    //}

    match args.get(1) {
        None => Err(Box::new(CommonError::new("Missing file parameter"))),
        Some(x) => split_file(x)
    }
}

fn split_file(filepath: &str) -> Result<(), Box<Error>>{
    let file = File::open(filepath)?;
    let mut reader = csv::Reader::from_reader(file);
    for record in reader.records() {
        let record = record?;
        println!("{:?}", record);
    }
    Ok(())
}
