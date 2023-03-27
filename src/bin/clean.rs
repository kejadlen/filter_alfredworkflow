extern crate plist;

use plist::Value;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let cursor = io::Cursor::new(buffer);
    let mut plist = Value::from_reader(cursor).unwrap();

    clean_vars(&mut plist);

    plist.to_writer_xml(io::stdout()).unwrap();
}

fn clean_vars(plist: &mut Value) {
    let dict = plist.as_dictionary_mut().unwrap();
    let vars = variables_dont_export(dict);

    let variables = dict.get_mut("variables")
                            .and_then(|x| x.as_dictionary_mut())
                            .unwrap();
    for var in vars {
        variables.insert(var.clone(), Value::String("".into()));
    }
}

fn variables_dont_export(dict: &plist::Dictionary) -> Vec<String> {
    dict.get("variablesdontexport")
        .and_then(|x| x.as_array())
        .map(|x| {
            x.iter()
             .map(|x| x.as_string().unwrap().into())
             .collect::<Vec<_>>()
        })
        .unwrap_or_else(Vec::new)
}
