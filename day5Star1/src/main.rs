use std::fs::File;
use std::io::prelude::*;
use std::borrow::Cow;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", remove_binome(&contents.trim()));
    Ok(())
}

fn remove_binome<'a>(s: &'a str) -> Cow<'a, str> {
    let mut buf = String::with_capacity(s.len());
    buf.push_str(s);
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
        println!("{}", buf.len());
        if !reacted {
            break;
        }
    }
    return Cow::Owned(buf);
}
