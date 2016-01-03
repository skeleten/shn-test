extern crate shn;
extern crate encoding;

use std::io::*;
use std::fs::File;

fn main() {
    let mut path = String::new();
    print!("Path to file: ");
    stdout().flush().ok();
    stdin().read_line(&mut path).ok();
    path = path.trim().to_string();
    println!("Trying to open {} now.", path);
    let f = File::open(path).unwrap();
    let file = match  shn::ShnReader::read_from(f, encoding::all::ASCII) {
        Ok(f) => f,
        Err(e) => {
            println!("Couldn't read file ({:?})", e);
            return;
        }
    };
    for r in file.data.iter() {
        println!("{:?}", r.data);
        stdin().read_line(&mut String::new()).ok();
    }
}
