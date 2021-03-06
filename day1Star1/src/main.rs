use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().map(|l| parse(&l.unwrap()));
    let sum : i32 = lines.sum();
    println!("{}", sum);
    Ok(())
}

fn parse(s :&str) -> i32 {
    let i : i32 = s[1..].parse().unwrap();
    if s.starts_with("+") {
        i
    } else {
        -1 * i
    }
}
