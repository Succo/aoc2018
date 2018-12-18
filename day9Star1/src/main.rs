extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use regex::Regex;


fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let re = Regex::new(r"(.*) players; last marble is worth (.*) points")
        .unwrap();

    let caps = re.captures(&contents).unwrap();
    let player = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let score = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    println!("{} {}", player, score);
    Ok(())
}

