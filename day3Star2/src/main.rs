use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// Matches # 123 @ 3,2: 5x4
// With    # id  @ x_coord,y_coord: width x heigth
#[derive(Debug)]
struct Claim {
    id: i64,
    x_coord: i64,
    y_coord: i64,
    width: i64,
    height: i64,
}

impl Claim {
    fn new(s: &str) -> Self {
        let mut iter = s.split_whitespace();
        let id_str = iter.next().unwrap();
        let id: i64 = id_str[1..].parse().unwrap();
        iter.next(); // drop the @
        let coords = iter.next().unwrap();
        let pt = coords.find(',').unwrap();
        let x_coord: i64 = coords[..pt].parse().unwrap();
        let y_coord: i64 = coords[pt+1..coords.len()-1].parse().unwrap();
        let dim = iter.next().unwrap();
        let mid = dim.find('x').unwrap();
        let width: i64 = dim[..mid].parse().unwrap();
        let height: i64 = dim[mid+1..].parse().unwrap();
        Claim{
            id: id,
            x_coord: x_coord,
            y_coord: y_coord,
            width: width,
            height: height,
        }
    }
    fn contains(&self, x: i64, y: i64) -> bool {
        return !(x <= self.x_coord || x > self.x_coord + self.width ||
            y <= self.y_coord || y > self.y_coord + self.height)
    }
}


fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines().map(|l| Claim::new(&l.unwrap()));
    let claims: Vec<_> = lines.collect();
    let mut overlap = HashSet::new();
    for x in 0..1000 {
        for y in 0..1000 {
            let mut contains: Vec<i64> = Vec::new();
            for c in claims.iter() {
                if c.contains(x, y) {
                    contains.push(c.id);
                }
            }
            if contains.len() > 1 {
                for id in contains.iter() {
                    overlap.insert(*id);
                }
            }
        }
    }
    for c in claims.iter() {
        if !overlap.contains(&c.id) {
            println!("{}", c.id)
        }
    }
    Ok(())
}
