use std::fs::File;
use std::io::prelude::*;

fn log_something(filename: &'static str, string: &'static [u8; 12]) -> std::io::Result<()> {
    let mut f = File::create(filename).unwrap();
    f.write_all(string)?;
    Ok(())
}

fn main() {
    match log_something("log.txt", b"ITS ALIVE!!!") {
        Ok(..) => println!("File created!"),
        Err(..) => println!("Error: could not create file."),
    }
}
