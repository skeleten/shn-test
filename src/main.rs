extern crate shn;
extern crate encoding;

use std::io::*;
use std::fs::File;

use shn::*;

fn main() {
    let path = get_file_path();
    println!("Trying to open {} now.", path);
    let f = File::open(path).unwrap();
    let enc = get_encoding();
    let file = match  shn::ShnReader::read_from(f, enc) {
        Ok(f) => f,
        Err(e) => {
            println!("Couldn't read file ({:?})", e);
            return;
        }
    };
    for r in file.data.iter() {
        print_row(r);
        println!("");
        stdout().flush().ok();
        // stdin().read_line(&mut String::new()).ok();
    }
    println!("");
}

fn print_row(row: &ShnRow) {
    for c in row.data.iter() {
        print_cell(c);
    }
}

fn get_encoding() -> &'static encoding::types::EncodingRef {
    const DEFAULT_NAME: &'static str = "ASCII";
    let encoding_name;
    let mut args = std::env::args();
    let mut found_arg = false;
    while let Some(s) =  args.next() {
        if s == "--encoding" {
            found_arg = true;
            break;
        }
    }
    if found_arg {
        if let Some(s) = args.next() {
            encoding_name = s.to_string();
        } else {
            panic!("Not enough arguments!");
        }
    } else {
        encoding_name = DEFAULT_NAME.to_string();
    }
    for enc in encoding::all::encodings() {
        if enc.name() == encoding_name {
            return enc;
        }
    }
    panic!("Encoding not found!");
}

fn get_file_path() -> String {
    let mut args = std::env::args();
    let mut found_path = false;
    while let Some(s) = args.next() {
        if s == "--path" {
            found_path = true;
            break;
        }
    }
    if found_path {
        args.next().unwrap().to_string()
    } else {
        let mut path = String::new();
        print!("Path to file: ");
        stdout().flush().ok();
        stdin().read_line(&mut path).ok();
        path = path.trim().to_string();
        path
    }
}

fn print_cell(cell: &ShnCell) {
    // we kinda print all the values in a tabbed format
    match cell {
        &ShnCell::StringFixedLen(ref s) => print!("{}\t", s),
        &ShnCell::StringZeroTerminated(ref s) => print!("{}\t", s),
        &ShnCell::Byte(b) => print!("{}\t", b),
        &ShnCell::SignedByte(b) => print!("{}\t", b),
        &ShnCell::UnsignedShort(s) => print!("{}\t", s),
        &ShnCell::SignedShort(s) => print!("{}\t", s),
        &ShnCell::UnsignedInteger(i) => print!("{}\t", i),
        &ShnCell::SignedInteger(i) => print!("{}\t", i),
        _ => { }
    }
}


