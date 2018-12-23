extern crate regex;

use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

struct Circle {
    marbles: Vec<usize>,
    active: usize,
}

impl Circle {
    fn new(size: usize) -> Self {
        let mut marbles = Vec::with_capacity(size);
        marbles.push(0);
        Circle{
            marbles: marbles,
            active: 0,
        }
    }
    fn insert_marble(&mut self, i: usize) -> usize {
        let len = self.marbles.len();
        if i % 23 == 0 {
            let pos = (self.active + len - 7) % len;
            let score = self.marbles.remove(pos);
            self.active = pos;
            score + i
        } else {
            let mut pos = (self.active + 2) % len;
            if pos == 0 {
                pos = len;
            }
            self.marbles.insert(pos, i);
            self.active = pos;
            0
        }
    }
}

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
    let mut players = vec![0; player];
    let mut circle = Circle::new(score);
    for marble in 1..score+1 {
        let score = circle.insert_marble(marble);
        players[(marble - 1) % player] += score;
    }
    println!("{}", players.iter().max().unwrap());

    Ok(())
}

