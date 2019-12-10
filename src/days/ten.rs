use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use thiserror::Error;
use itertools::Itertools;
use std::fmt;

use crate::problem::Problem;

// The input is only 20 by 20 so we don't need primes larger than this.
const PRIMES: [isize; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum SpaceObject {
    Space,
    Asteroid
}

#[derive(Debug, Error)]
enum ParseError {
    #[error("`{0}` is not a valid space object.")]
    InvalidObject(String)
}

impl FromStr for SpaceObject {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(SpaceObject::Space),
            "#" => Ok(SpaceObject::Asteroid),
            _ => Err(ParseError::InvalidObject(s.to_string()))
        }
    }
}

fn str2map(s: &str) -> Result<HashMap<Point, SpaceObject>, ParseError> {
    let mut map = HashMap::new();

    for (y, line) in s.split_whitespace().enumerate() {
        for x in 0..line.len() {
            let obj = SpaceObject::from_str(line.get(x..x + 1).unwrap())?;
            map.insert(
                Point { x: x as isize, y: y as isize },
                obj
            );
        }
    }

    Ok(map)
}

fn factors(n: &isize) -> HashSet<isize> {
    let mut fs = HashSet::new();
    for p in &PRIMES {
        if n % p == 0 {
            fs.insert(*p);
            fs.insert(n / p);
        }
    }
    fs
}

fn points_obscuring(station: &Point, target: &Point) -> Vec<Point> {
    let dx = target.x - station.x;
    let dy = target.y - station.y;

    let mut blockers = Vec::new();
    for factor in factors(&dx).intersection(&factors(&dy)) {
        blockers.push(Point{ x: dx / factor, y: dy / factor });
    }

    blockers
}

fn points_visible_from(station: Point, map: &HashMap<Point, SpaceObject>) -> usize {
    let mut count = 0;

    for (point, target) in map {
        match target {
            SpaceObject::Space => { },
            SpaceObject::Asteroid => {
                let blockers = points_obscuring(&station, point);
                println!("{}", &blockers.iter().join(" "));
                if blockers.iter().all(|b| map[b] == SpaceObject::Space) {
                    count += 1;
                }
            }
        }
    }

    count
}

const EXAMPLE: &str = ".#..#
.....
#####
....#
...##";

pub struct DayTen {}

impl Problem for DayTen {
    fn part_one(&self, input: &str) -> String {
        let map = str2map(EXAMPLE).unwrap();

        for x in 0..5 {
            for y in 0..5 {
                let station = Point { x: x, y: y };
                match map[&station] {
                    SpaceObject::Space => print!("."),
                    SpaceObject::Asteroid => print!("{}", points_visible_from(station, &map))
                }
            }
            println!("");
        }

        format!("{}", "Part one not yet implemented.")
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
