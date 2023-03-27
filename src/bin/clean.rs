use color_eyre::eyre::{anyhow, Result};
use plist::Value;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let cursor = io::Cursor::new(buffer);
    let mut plist = Value::from_reader(cursor)?;

    clean_vars(&mut plist)?;

    plist.to_writer_xml(io::stdout())?;

    Ok(())
}

fn clean_vars(plist: &mut Value) -> Result<()> {
    let dict = plist
        .as_dictionary_mut()
        .ok_or_else(|| anyhow!("error opening plist"))?;
    let vars = variables_dont_export(dict);
    if vars.is_empty() {
        return Ok(());
    }

    let variables = dict
        .get_mut("variables")
        .and_then(|x| x.as_dictionary_mut())
        .ok_or_else(|| anyhow!(""))?;
    for var in vars {
        variables.insert(var.clone(), Value::String("".into()));
    }

    Ok(())
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
