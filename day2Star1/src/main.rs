use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().map(|l| count_letters(&l.unwrap()));
    let sum :(u32, u32) =lines.fold((0, 0), |acc , x| (acc.0 +x.0, acc.1 + x.1));
    println!("{}", sum.0*sum.1);
    Ok(())
}

fn count_letters(s :&str) -> (u32, u32) {
    let mut bytes = s.bytes();
    let mut letters = HashMap::new();
    while let Some(i) = bytes.next() {
        let val = match letters.get(&i) {
            Some(c) => c+1,
            None => 1,
        };
        letters.insert(i, val);
    };
    let mut res = (0, 0);
    for (_key, val) in letters.iter() {
        if *val == 2 {
            res = (1, res.1)
        }
        if *val == 3 {
            res = (res.0, 1)
        }
    }
    res
}
