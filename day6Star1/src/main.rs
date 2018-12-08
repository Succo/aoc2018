use std::collections::HashMap;
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
    fn equal(&self, pt: &Coordinates) -> bool {
        self.x == pt.x && self.y == pt.y
    }
    fn get_closest(&self, points: &Vec<Coordinates>) -> Option<Coordinates> {
        let mut res = &self.clone();
        let mut d = self.dist(&points[0]);
        let mut is_valid = false;
        for pt in points.iter() {
            if self.equal(pt) {
                return Some(pt.clone());
            }
            if self.dist(pt) < d {
                res = pt;
                d = self.dist(pt);
                is_valid = true;
            } else if self.dist(pt) == d {
                is_valid = true;
            }
        }
        match is_valid {
            true => Some(res.clone()),
            false => None,
        }
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

// (min_x, max_x, min_y, max_y)
type Bbox = (isize, isize, isize, isize);

fn main() -> std::io::Result<()> {
    let file = File::open("input")?;
    let buf_reader = BufReader::new(file);
    let points: Vec<_> = buf_reader.lines().map(|l| l.unwrap()
                                           .parse::<Coordinates>()
                                           .unwrap())
        .collect();
    let bbox = get_bounding_box(&points);
    //println!("{:?}", bbox);
    //let p0 = points[0].clone();
    //println!("{:?}", escapes_bbox(bbox, points, p0));

    let mut closests: HashMap<Coordinates, isize> = HashMap::new();
    for x in bbox.0..bbox.1 {
        for y in bbox.2..bbox.3 {
            let pt = Coordinates{x: x, y: y};
            let closest = pt.get_closest(&points);
            if closest == None {
                continue;
            }
            let c_pt = closest.unwrap();
            let val = match closests.get(&c_pt) {
                Some(c) => c + 1,
                None => 1,
            };
            closests.insert(c_pt, val);
        }
    }

    let mut area = 0;
    for (pt, size) in closests.iter() {
        if size > &area && !escapes_bbox(bbox, &points, pt) {
            area = *size;
        }
    }
    println!("{}", area);
    Ok(())
}

// returns (min_x, max_x, min_y, max_y)
fn get_bounding_box(points: &Vec<Coordinates>) -> Bbox {
    let init = (points[0].x, points[0].x, points[0].y, points[0].y);
    points.iter().fold(init, |acc, point| (acc.0.min(point.x) - 10,
                                           acc.0.max(point.x) + 10,
                                           acc.1.min(point.y) - 10,
                                           acc.1.max(point.y) + 10))
}

fn escapes_bbox(bbox: Bbox, points: &Vec<Coordinates>, point: &Coordinates) -> bool {
    let borders = vec![
        Coordinates { x: bbox.0, y: point.y },
        Coordinates { x: bbox.1, y: point.y },
        Coordinates { x: point.x, y: bbox.2 },
        Coordinates { x: point.x, y: bbox.3 }
    ];
    for pt in borders.iter() {
        let d1 = point.dist(pt);
        let mut d2 = d1 + 1;
        for position in points.iter() {
            if point.equal(position) {
                continue;
            }
            d2 = d2.min(position.dist(pt));
        }
        if d1 < d2 {
            return true;
        }

    }
    false
}
