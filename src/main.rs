#[macro_use]
extern crate lazy_static;

use std::io::{self, BufReader, BufWriter};

mod hex;

fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    if let Err(e) = hex::hex_to_b64(&mut reader, &mut writer) {
        panic!("=( {}", e);
    }
}