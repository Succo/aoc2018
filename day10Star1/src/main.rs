extern crate regex;
#[macro_use] extern crate lazy_static;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

use regex::Regex;

struct Point {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Point {
    fn update(&mut self, t: isize) {
        self.x += t * self.vx;
        self.y += t * self.vy;
    }
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"position=< ?(.*),  ?(.*)> velocity=< ?(.*),  ?(.*)>")
                .unwrap();
        }
        let caps = RE.captures(&s).unwrap();

        let x = caps.get(1).unwrap().as_str().parse::<isize>()?;
        let y = caps.get(2).unwrap().as_str().parse::<isize>()?;
        let vx = caps.get(3).unwrap().as_str().parse::<isize>()?;
        let vy = caps.get(4).unwrap().as_str().parse::<isize>()?;

        Ok(Point{x: x, y: y, vx: vx, vy: vy})
    }
}

// (min_x, max_x, min_y, max_y)
#[derive(Debug)]
struct Bbox((isize, isize, isize, isize));

impl Bbox {
    fn new(points: &Vec<Point>) -> Self {
        let init = (points[0].x, points[0].x, points[0].y, points[0].y);
        Bbox(points.iter().fold(init, |acc, point| (
                acc.0.min(point.x),
                acc.1.max(point.x),
                acc.2.min(point.y),
                acc.3.max(point.y)
            ))
        )
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let mut points: Vec<_> = buf_reader.lines().map(|l| l.unwrap()
                                           .parse::<Point>()
                                           .unwrap())
        .collect();

    println!("{}", points.len());
    loop {
        for pt in points.iter_mut() {
            pt.update(1000)
        }
        let bbox = Bbox::new(&points);
        println!("{:?}", bbox);
        if (bbox.0).1 - (bbox.0).0 < 1000 {
            break
        }
    }
    println!("{:?}", Bbox::new(&points));
    Ok(())
}

