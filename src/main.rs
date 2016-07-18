extern crate plist;

use plist::Plist;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let cursor = io::Cursor::new(buffer);
    let plist = Plist::read(cursor).unwrap();

    let dict = match plist {
                   Plist::Dictionary(dict) => Some(dict),
                   _ => None,
               }
               .unwrap();
    let vars = match dict.get("variablesdontexport") {
                   Some(&Plist::Array(ref array)) => Some(array),
                   _ => None,
               }
               .unwrap();

    println!("{:?}", vars);
}
