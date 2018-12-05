extern crate chrono;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use chrono::{NaiveDateTime,Timelike};

enum Action {
    WakesUp,
    FallsAsleep,
    NewShift(isize),
}

struct ListItem {
    date: chrono::NaiveDateTime,
    action: Action
}

impl ListItem {
    fn new(s: &str) -> Self {
        let date_str = &s[1..17];
        let d = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M").unwrap();
        let action_str = &s[19..];
        let a = match action_str {
            "wakes up" => Action::WakesUp,
            "falls asleep" => Action::FallsAsleep,
            _ => Action::NewShift(parse_guard_id(action_str)),
        };
        ListItem{
            date: d,
            action: a,
        }
    }
}

fn parse_guard_id(action_str: &str) -> isize {
    let mut iter = action_str.split_whitespace();
    iter.next();
    let id_str = iter.next().unwrap();
    id_str[1..].parse().unwrap()
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().map(|l| ListItem::new(&l.unwrap()));
    let mut lists: Vec<_> = lines.collect();
    lists.sort_by(|a, b| a.date.cmp(&b.date));

    let mut active = 0;
    let mut times: HashMap<isize, Vec<usize>> = HashMap::new();
    for (i, info) in lists.iter().enumerate() {
        match info.action {
            Action::WakesUp => (
                if active != 0 {
                    let mut s_times = times.entry(active).or_insert(vec![0; 59]);
                    let prev_d = lists[i-1].date;
                    for t in prev_d.minute() as usize..info.date.minute() as usize {
                        s_times[t] = s_times[t] + 1
                    }
                }),
            Action::FallsAsleep => (),
            Action::NewShift(id) => active = id,
        }
    }
    let max_s = times.iter().
        max_by(|x, y| get_max_time(x.1).cmp(&get_max_time(y.1)));
    let max_t = times.get(max_s.unwrap().0).unwrap().iter().enumerate().
        max_by(|x, y| x.1.cmp(y.1));
    println!("{}", max_t.unwrap().0 as isize * max_s.unwrap().0);
    Ok(())
}

fn get_max_time(v: &std::vec::Vec<usize>) -> usize {
    *v.iter().enumerate().max_by(|x, y| x.1.cmp(y.1)).unwrap().1
}

