extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

use regex::Regex;

struct Circle {
    marbles: VecDeque<usize>,
}

impl Circle {
    fn new(size: usize) -> Self {
        let mut marbles = VecDeque::with_capacity(size);
        marbles.push_front(0);
        Circle{
            marbles: marbles,
        }
    }
    fn insert_marble(&mut self, i: usize) {
        let m1 = self.marbles.pop_front().unwrap();
        self.marbles.push_back(m1);
        let m2 = self.marbles.pop_front().unwrap();
        self.marbles.push_back(m2);
        self.marbles.push_front(i);
    }
    fn remove_marble(&mut self, i: usize) -> usize {
        for _i in 0..6 {
            let m = self.marbles.pop_back().unwrap();
            self.marbles.push_front(m);
        }
        let score = self.marbles.pop_back().unwrap();
        score + i
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
    let score = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() * 100;
    println!("{} {}", player, score);
    let mut players = vec![0; player];
    let mut circle = Circle::new(score);
    let mut modulo = 0;
    for marble in 1..score+1 {
        modulo += 1;
        if modulo == 23 {
            let score = circle.remove_marble(marble);
            players[(marble - 1) % player] += score;
            modulo = 0;
        } else {
            circle.insert_marble(marble)
        }

    }
    println!("{}", players.iter().max().unwrap());

    Ok(())
}

