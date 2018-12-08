use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let poly = contents.trim();
    let mut size =  "abcdefghijklmnopqrstuvwxyz".chars().map(|c| size_without_binome(c, &poly));
    println!("{}", size.min().unwrap());
    Ok(())
}

fn size_without_binome(excluded: char, s: &str) -> usize {
    let mut buf = String::with_capacity(s.len());
    for c in s.chars() {
        if c.to_lowercase().next().unwrap() != excluded {
            buf.push(c);
        }
    }
    loop {
        let mut res = String::with_capacity(buf.len()/2);
        let mut reacted = false;
        {
            let b = buf.as_bytes();
            let mut i = 0;
            while i < b.len() {
                if i == b.len()-1 {
                    res.push(b[i] as char);
                    break;
                }
                if b[i] != b[i+1] && ( b[i].to_ascii_lowercase() == b[i+1] || b[i].to_ascii_uppercase() == b[i+1] ) {
                    reacted = true;
                    i = i+2;
                } else {
                    res.push(b[i] as char);
                    i = i+1;
                }
            }
        }
        buf = res;
        if !reacted {
            break;
        }
    }
    return buf.len()
}
