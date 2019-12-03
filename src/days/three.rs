use std::fmt;
use std::error::Error;
use std::collections::HashSet;
use std::iter::FromIterator;

use crate::problem::Problem;

pub struct DayThree {}

#[derive(Debug)]
enum ParseError {
    InvalidDirection(String),
    InvalidDistance(String)
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidDirection(dir) =>
                write!(f, "{} is not a valid direction", dir),
            ParseError::InvalidDistance(dist) =>
                write!(f, "{} is not a valid distance", dist)
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn from_str(s: &str) -> Result<Direction, ParseError> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(ParseError::InvalidDirection(s.to_string()))
        }
    }
}

struct Move {
    direction: Direction,
    distance: usize
}

impl Move {
    fn from_str(s: &str) -> Result<Move, ParseError> {
        let (dir_str, dist_str) = s.split_at(1);
        let dist = usize::from_str_radix(dist_str.trim(), 10)
            .map_err(|_| ParseError::InvalidDistance(dist_str.to_string()))?;

        Ok(Move {
            direction: Direction::from_str(dir_str)?,
            distance: dist
        })
    }

    fn move_list_from_str(s: &str) -> Result<Vec<Move>, ParseError> {
        let mut moves = Vec::new();
        for token in s.split(",") {
            moves.push(Move::from_str(token)?);
        }
        Ok(moves)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize
}

impl Point {
    fn manhattan_magnitude(&self) -> usize {
        (self.x.abs() as usize) + (self.y.abs() as usize)
    }
}

fn extend_point(anchor: Point, step: &Move) -> Vec<Point> {
    match step.direction {
        Direction::Up => {
            (1..step.distance + 1)
                .map(|dy| Point { x: anchor.x, y: anchor.y - (dy as isize) })
                .collect()
        },
        Direction::Down => {
            (1..step.distance + 1)
                .map(|dy| Point { x: anchor.x, y: anchor.y + (dy as isize) })
                .collect()
        },
        Direction::Left => {
            (1..step.distance + 1)
                .map(|dx| Point { x: anchor.x - (dx as isize), y: anchor.y })
                .collect()
        },
        Direction::Right => {
            (1..step.distance + 1)
                .map(|dx| Point { x: anchor.x + (dx as isize), y: anchor.y })
                .collect()
        }
    }
}

fn spaces_traversed(moves: &[Move]) -> Vec<Point> {
    let mut points = Vec::new();
    let mut turtle = Point { x: 0, y: 0 };

    for step in moves {
        let new_points = extend_point(turtle, step);
        // Looked at input, there are no 0 moves so this should always work.
        turtle = new_points.get(new_points.len() - 1).unwrap().clone();
        points.extend(new_points);
    }

    points
}

fn find_index(points: &Vec<Point>, target: &Point) -> Option<usize> {
    for (i, point) in points.iter().enumerate() {
        if point == target {
            return Some(i);
        }
    }
    None
}

impl Problem for DayThree {
    fn part_one(&self, input: &str) -> String {
        let lines: Vec<&str> = input.split_whitespace().collect();

        // Assume two lines by problem definition.
        let mut spaces_one = HashSet::new();
        spaces_one.extend(
            spaces_traversed(&Move::move_list_from_str(lines[0]).unwrap())
        );
        let mut spaces_two = HashSet::new();
        spaces_two.extend(
            spaces_traversed(&Move::move_list_from_str(lines[1]).unwrap())
        );

        let closest_crossing = spaces_one.intersection(&spaces_two)
            .min_by_key(|p| p.manhattan_magnitude())
            // Assume there's at least one crossing by problem definition.
            .unwrap();

        format!("{}", closest_crossing.manhattan_magnitude())
    }

    fn part_two(&self, input: &str) -> String {
        let lines: Vec<&str> = input.split_whitespace().collect();

        let spaces_one = spaces_traversed(
            &Move::move_list_from_str(lines[0]).unwrap()
        );
        let spaces_two = spaces_traversed(
            &Move::move_list_from_str(lines[1]).unwrap()
        );

        let mut set_one: HashSet<Point> = HashSet::new();
        set_one.extend(&spaces_one);
        let mut set_two = HashSet::new();
        set_two.extend(&spaces_two);

        let fastest_crossing = set_one.intersection(&set_two)
            .min_by_key(|p| find_index(&spaces_one, p).unwrap() + find_index(&spaces_two, p).unwrap())
            .unwrap();

        format!(
            "{}",
              find_index(&spaces_one, fastest_crossing).unwrap()
            + find_index(&spaces_two, fastest_crossing).unwrap()
            // 2 here to account for find_index being 0-indexed.
            + 2
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traversal_intersection_example() -> Result<(), ParseError> {
        let moves_one = Move::move_list_from_str("R8,U5,L5,D3")?;
        let moves_two = Move::move_list_from_str("U7,R6,D4,L4")?;

        let mut spaces_one = HashSet::new();
        spaces_one.extend(spaces_traversed(&moves_one));
        let mut spaces_two = HashSet::new();
        spaces_two.extend(spaces_traversed(&moves_two));

        let common: Vec<Point> = spaces_one.intersection(&spaces_two)
            .cloned()
            .collect();

        assert_eq!(vec![Point{ x: 3, y: -3 }, Point{ x: 6, y: -5 }], common);

        Ok(())
    }

    #[test]
    fn manhattan_magnitude() {
        assert_eq!(0, Point{ x: 0, y: 0}.manhattan_magnitude());
        assert_eq!(4, Point{ x: 1, y: 3}.manhattan_magnitude());
        assert_eq!(5, Point{ x: -2, y: 3}.manhattan_magnitude());
        assert_eq!(2, Point{ x: -1, y: -1}.manhattan_magnitude());
        assert_eq!(6, Point{ x: 2, y: -4}.manhattan_magnitude());
    }
}
