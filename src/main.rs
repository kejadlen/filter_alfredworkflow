extern crate plist;

use plist::{self, Plist};
use std::collections::btree_map::BTreeMap;
use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let cursor = io::Cursor::new(buffer);
    let mut plist = Plist::read(cursor).unwrap();

    let mut dict = match plist {
                       Plist::Dictionary(ref mut dict) => Some(dict),
                       _ => None,
                   }
                   .unwrap();
    let vars = variables_dont_export(&dict);

    let mut variables = match dict.get_mut("variables") {
                            Some(&mut Plist::Dictionary(ref mut dict)) => {
                                Some(dict)
                            }
                            _ => None,
                        }
                        .unwrap();
    for var in vars {
        variables.insert(var.clone(), Plist::String("".into()));
    }

    // println!("{:?}", dict);
    println!("{:?}", variables);

    // io::stdout().write(b"hello world").unwrap();
    let serializer = plist::Serializer::new(io::stdout());
    serializer.write(dict);
}

fn variables_dont_export(dict: &BTreeMap<String, Plist>) -> Vec<String> {
    match dict.get("variablesdontexport") {
        Some(&Plist::Array(ref array)) => Some(array),
        _ => None,
    }
    .unwrap()
    .iter()
    .map(|x| {
        match x {
            &Plist::String(ref string) => Some(string),
            _ => None,
        }
        .unwrap()
        .clone()
    })
    .collect::<Vec<_>>()
}
