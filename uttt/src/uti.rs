use std::collections::HashMap;

type ParseResult = Result<HashMap<String, String>, String>;

pub fn parse_info(line: &str) -> ParseResult {
    if line.len() < 5 || &line[..5] != "info " {
        return Err(String::from("expected command to start with 'info'"));
    }
    parse_keyvalue(&line[5..])
}

// parse a key-value pair string and return the corresponding HashMap
fn parse_keyvalue(line: &str) -> ParseResult {
    let mut map = HashMap::new();
    let tokens = line.split(";");
    for tok in tokens {
        //let tok = tok.trim();
        let kv = tok.split("=").collect::<Vec<&str>>();
        if kv.len() != 2 {
            return Err(String::from("invalid key-value pair format"));
        }
        map.insert(String::from(kv[0]), String::from(kv[1]));
    }
    Ok(map)
}
