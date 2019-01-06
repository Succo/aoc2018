use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const MAX: usize = 50000000000;

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();
    let l1 = lines.next().unwrap().unwrap();
    let mut state = l1.get(15..).unwrap().to_string();

    lines.next();

    let valids: HashSet<_> = lines.filter_map(|l| parse(&l.unwrap()))
        .collect();
    let mut seen = HashMap::new();

    let mut min: isize = 0;
    for i in 0..MAX {
        let mut size: usize = state.len();
        state = state.trim_left_matches('.').to_string();
        min += size as isize - state.len() as isize;
        state = state.trim_right_matches('.').to_string();
        size = state.len();

        {
        let prev = seen.get(&state.to_string());
        if let Some((prev_i, prev_min)) = prev {
            let step = i - prev_i;
            let todo = MAX - i;
            // let's hope it's a round number
            let iter: isize = (todo/step) as isize;
            min += iter * ( min - prev_min );
            break
        }
        }

        seen.insert(state.to_string(), (i, min));

        // Adds some padding to state
        let mut s = String::from("....");
        s.push_str(state.as_str());
        s.push_str("....");

        let mut new_state = String::with_capacity(size + 4);
        // iterate on all char of new_state
        for j in 0..size+4 {
            let parent = s.get(j..j+5).unwrap();
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
