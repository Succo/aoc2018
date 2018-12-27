use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();
    let l1 = lines.next().unwrap().unwrap();
    let input = l1.as_str().get(15..).unwrap();
    println!("{}", input);

    lines.next();

    let valids: HashSet<_> = lines.filter_map(|l| parse(&l.unwrap()))
        .collect();
    println!("{:?}", valids);
    Ok(())
}

fn parse(s: &str) -> Option<std::string::String> {
    let is_valid = &s[9..10];
    if is_valid == "." {
        return None
    }
    let key = &s[0..5];
    return Some(key.to_string())
}
