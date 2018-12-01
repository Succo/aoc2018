use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let mut inputs = buf_reader.lines().map(|l| parse(&l.unwrap()));
    let mut v: Vec<i32> = Vec::new();
    while let Some(i) = inputs.next() {
        v.push(i)
    }
    let mut freqs = v.into_iter().cycle();
    let mut seen = HashSet::new();
    let mut sum = 0;
    while !seen.contains(&sum) {
        seen.insert(sum);
        sum = sum + freqs.next().unwrap();
    }

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
