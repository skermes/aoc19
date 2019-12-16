use std::collections::HashMap;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::problem::Problem;
use crate::intcode::{Machine,OperationalError};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize
}

impl Point {
    fn in_direction(&self, dir: &Direction) -> Point {
        match dir {
            Direction::North => Point { x: self.x, y: self.y - 1 },
            Direction::South => Point { x: self.x, y: self.y + 1 },
            Direction::West => Point { x: self.x - 1, y: self.y },
            Direction::East => Point { x: self.x + 1, y: self.y }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Space {
    Empty,
    Wall,
    Oxygen,
    Unknown
}

impl Space {
    fn from_int(i: isize) -> Space {
        match i {
            0 => Space::Wall,
            1 => Space::Empty,
            _ => Space::Oxygen
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    fn as_int(&self) -> isize {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            _ => Direction::East
        }
    }
}

#[derive(Debug)]
struct RepairDroid {
    program: Machine,
    maze: HashMap<Point, Space>,
    position: Point,
    step_count: usize
}

impl RepairDroid {
    fn new(program: Machine) -> Self {
        RepairDroid {
            program: program,
            maze: HashMap::new(),
            position: Point { x: 0, y: 0 },
            step_count: 0
        }
    }

    fn space_at(&self, point: &Point) -> &Space {
        match self.maze.get(point) {
            Some(space) => space,
            None => &Space::Unknown
        }
    }

    fn run(&mut self) -> Result<(), OperationalError> {
        loop {
            let dir: Direction = rand::random();
            let next_point = self.position.in_direction(&dir);
            self.program.write(dir.as_int());
            self.program.run()?;
            let space = Space::from_int(self.program.read()[0]);
            self.maze.insert(next_point, space);

            if space != Space::Wall {
                self.position = next_point;
            }

            if space == Space::Oxygen {
                // break;
            }

            self.step_count += 1;

            if self.step_count > 100000 {
                break;
            }
        }

        Ok(())
    }
}

pub struct DayFifteen {}

impl Problem for DayFifteen {
    fn part_one(&self, input: &str) -> String {
        let program = Machine::from_str(input).unwrap();
        let mut droid =RepairDroid::new(program);
        droid.run().unwrap();

        let min_x = droid.maze.keys().map(|p| p.x).min().unwrap();
        let min_y = droid.maze.keys().map(|p| p.y).min().unwrap();
        let max_x = droid.maze.keys().map(|p| p.x).max().unwrap();
        let max_y = droid.maze.keys().map(|p| p.y).max().unwrap();

        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                if x == 0 && y == 0 {
                    print!("0");
                } else {
                    match droid.space_at(&Point { x: x, y: y }) {
                        Space::Empty => print!(" "),
                        Space::Wall => print!("#"),
                        Space::Oxygen => print!("*"),
                        Space::Unknown => print!(".")
                    }
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
