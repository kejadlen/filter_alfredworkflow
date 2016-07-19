extern crate plist;

use plist::Plist;
use std::collections::btree_map::BTreeMap;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let cursor = io::Cursor::new(buffer);
    let mut plist = Plist::read(cursor).unwrap();

    clean_vars(&mut plist);

    let mut event_writer = plist::xml::EventWriter::new(io::stdout());
    for event in plist.into_events() {
        event_writer.write(&event).unwrap();
    }
}

fn clean_vars(plist: &mut Plist) {
    let mut dict = plist.as_dictionary_mut().unwrap();
    let vars = variables_dont_export(&dict);

    let mut variables = dict.get_mut("variables")
                            .and_then(|x| x.as_dictionary_mut())
                            .unwrap();
    for var in vars {
        variables.insert(var.clone(), Plist::String("".into()));
    }
}

fn variables_dont_export(dict: &BTreeMap<String, Plist>) -> Vec<String> {
    dict.get("variablesdontexport")
        .and_then(|x| x.as_array())
        .map(|x| {
            x.iter()
             .map(|x| x.as_string().unwrap().into())
             .collect::<Vec<_>>()
        })
        .unwrap()
}
