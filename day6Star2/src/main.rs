extern crate rayon;
use rayon::prelude::*;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Coordinates {
    x: isize,
    y: isize
}

impl Coordinates {
    fn dist(&self, pt: &Coordinates) -> isize {
        (self.x - pt.x).abs() + (self.y - pt.y).abs()
    }
}

impl FromStr for Coordinates {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',')
                                 .collect();

        let x_fromstr = coords[0].parse::<isize>()?;
        let y_fromstr = coords[1].trim().parse::<isize>()?;

        Ok(Coordinates { x: x_fromstr, y: y_fromstr })
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let points: Vec<_> = buf_reader.lines().map(|l| l.unwrap()
                                           .parse::<Coordinates>()
                                           .unwrap())
        .collect();
    let size = points.len();
    let mut center = points.iter().fold(Coordinates{x: 0, y: 0}, |acc, pt| Coordinates{x: acc.x + pt.x, y: acc.y + pt.y});
    center.x = center.x / size as isize;
    center.y = center.y / size as isize;
    println!("{:?}", center);
    println!("{}", points.par_iter().map(|pt| center.dist(pt)).sum::<isize>());

    let mut total = 0;
    for size in 0.. {
        let mut star_total = 0;
        for point in get_star_shape(&center, size) {
            let point_dist: isize = points.par_iter().map(|pt| point.dist(pt)).sum();
            if point_dist < 10000 {
                star_total = star_total + 1;
            }
        }
        if star_total == 0 {
            break;
        }
        total = total + star_total;
    }

    println!("{}", total);
    Ok(())
}

fn get_star_shape(center: &Coordinates, size: isize) -> Vec<Coordinates> {
    if size == 0 {
        return vec![center.clone()];
    }
    let mut res = vec![];
    res.push(Coordinates{x: center.x + size, y: center.y});
    res.push(Coordinates{x: center.x - size, y: center.y});
    res.push(Coordinates{x: center.x, y: center.y + size});
    res.push(Coordinates{x: center.x, y: center.y - size});
    for i in 1..size {
        let j = size - i;
        res.push(Coordinates{x: center.x + i, y: center.y + j});
        res.push(Coordinates{x: center.x + i, y: center.y - j});
        res.push(Coordinates{x: center.x - i, y: center.y + j});
        res.push(Coordinates{x: center.x - i, y: center.y - j});
    }
    res
}
