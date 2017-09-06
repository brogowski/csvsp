extern crate csv;

#[macro_use]
extern crate quick_error;



use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use errors::CommonError::CommonError;

mod errors;

pub fn run(args: Vec<String>) -> Result<(), Box<Error>> {
    //PITFALL 0: - move in for loop
    //for x in &args {
    //    println!("{}", x);
    //}

    match args.get(1) {
        None => Err(Box::new(CommonError("Missing file parameter"))),
        Some(x) => split_file(x),
    }
}

fn split_file(filepath: &str) -> Result<(), Box<Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path(filepath)?;
    let mut iter = reader.records();
    let headers = iter.next();
    match headers {
        None => Err(Box::new(CommonError("No records to work with"))),
        Some(x) => {
            let mut threads = Vec::new();
            let mut inputs: Vec<Sender<String>> = Vec::new();
            let record = x?;
            for filename in record.iter() {               
                let (tx, rx) = mpsc::channel();
                let filename = vec![filename, ".txt"].join("");
                let thread = thread::spawn(move || {
                    let mut file = File::create(filename).unwrap();
                    for line in rx {
                        file.write_all(format!("{}\n", line).as_bytes())
                            .expect("Writing to file failed");
                    }
                });
                threads.push(thread);
                inputs.push(tx);
            }

            for record in iter {
                for (index, column) in record?.iter().enumerate() {
                    if let Some(input) = inputs.get(index){
                        let column = column.to_owned();
                        input.send(column).unwrap();
                    }
                }
            }

            std::mem::drop(inputs);
            
            for thread in threads {
                thread.join().unwrap();
            }
            
            Ok(())
        }
    }
}
