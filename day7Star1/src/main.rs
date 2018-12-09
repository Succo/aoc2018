extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();

    let mut sons: HashMap<char, Vec<char>> = HashMap::new();
    let re = Regex::new(r"Step (.) must be finished before step (.) can begin.")
        .unwrap();

    for line in lines {
        let l = &line.unwrap();
        let caps = re.captures(l).unwrap();
        let before = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let after = caps.get(2).unwrap().as_str().chars().next().unwrap();
        sons.entry(before).or_insert(vec![]);

        let mut befores = sons.entry(after).or_insert(vec![]);
        befores.push(before);
    }
    println!("{:?}", sons);
    for (_, l) in sons.iter_mut() {
        l.sort_unstable();
    }

    let mut s = String::new();
    while sons.len() > 0 {
        let key = get_valid(&sons);
        s.push(key);

        sons.remove(&key);

        for (_, l) in sons.iter_mut() {
            match l.binary_search(&key) {
                Ok(idx) =>  {l.remove(idx);},
                Err(_) => (),
            };
        }
    }

    println!("{}", s);
    Ok(())
}

fn get_valid(sons: &HashMap<char, Vec<char>>) -> char {
    let mut valids: Vec<_> = sons.iter().filter(|(_k, s)| s.len() == 0).collect();
    valids.sort_unstable_by(|a, b| a.0.cmp(b.0));
    *valids[0].0
}
