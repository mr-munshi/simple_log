#[macro_use]
extern crate nickel;

use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;

use nickel::Nickel;

fn formatted_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn record_entry_in_log(filename: &str, bytes: &[u8]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(filename)?;
    file.write_all(bytes)?;
    Ok(())
}

fn log_time(filename: &'static str) -> std::io::Result<String> {
    let entry = formatted_time_entry();
    {
        let bytes = entry.as_bytes();

        record_entry_in_log(filename, &bytes)?;
    }
    Ok(entry)
}

fn do_log_time() -> String {
    match log_time("log.txt") {
        Ok(..) => format!("File created!"),
        Err(e) => format!("Error: {}", e),
    }
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            do_log_time()
        }
    });

    server
        .listen("127.0.0.1:6767")
        .expect("Failed to launch server");
}
