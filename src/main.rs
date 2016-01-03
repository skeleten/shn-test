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
    let file = shn::ShnReader::read_from(f, encoding::all::UTF_8).ok().unwrap();
    for r in file.data.iter() {
        println!("{:?}", r);
        stdin().read_line(&mut String::new()).ok();
    }
}
