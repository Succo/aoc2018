use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();
    let l1 = lines.next().unwrap().unwrap();
    let mut state = l1.get(15..).unwrap().to_string();
    println!("{}", state.as_str());

    lines.next();

    let valids: HashSet<_> = lines.filter_map(|l| parse(&l.unwrap()))
        .collect();

    let mut min: isize = 0;
    // let mut new_state = String::new();
    for _i in 0..20 {
        let mut size: usize = state.len();
        state = state.trim_left_matches('.').to_string();
        min += size as isize - state.len() as isize;
        state = state.trim_right_matches('.').to_string();
        size = state.len();
        println!("{}", state);

        // Adds some padding to state
        let mut s = String::from("....");
        s.push_str(state.as_str());
        s.push_str("....");

        let mut new_state = String::with_capacity(size + 4);
        // iterate on all char of new_state
        for i in 0..size+4 {
            let parent = s.get(i..i+5).unwrap();
            if valids.contains(parent) {
                new_state.push('#');
            }else {
                new_state.push('.');
            }
        }
        min = min - 2;
        state = new_state;
    }

    println!("{:?}", state.as_str());
    println!("{}", min);
    let mut total = 0;
    for (i, c) in state.chars().enumerate() {
        if c == '#' {
            total += i + min as usize
        }
    }
    println!("{}", total);
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
