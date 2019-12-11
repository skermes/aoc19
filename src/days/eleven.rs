use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

use crate::problem::Problem;
use crate::intcode::{Machine,MachineState,OperationalError};

#[derive(Debug, Error)]
enum PaintingError {
    #[error("`{0}` is not a valid color.")]
    InvalidColor(isize),
    #[error("`{0}` is not a valid turn.")]
    InvalidTurn(isize),
    #[error("Error encountered while running intcode program.")]
    IntcodeError(#[from] OperationalError),
    #[error("Brain is still running after `run` exited.")]
    BrainStillRunning
}

type PaintingResult<T> = Result<T, PaintingError>;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Point {
    x: isize,
    y: isize
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Color {
    Black,
    White
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Color::Black => " ",
            Color::White => "#"
        })
    }
}

impl Color {
    fn from_int(i: isize) -> PaintingResult<Color> {
        match i {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => Err(PaintingError::InvalidColor(i))
        }
    }

    fn to_int(&self) -> isize {
        match self {
            Color::Black => 0,
            Color::White => 1
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Turn {
    Left,
    Right
}

impl Turn {
    fn from_int(i: isize) -> PaintingResult<Turn> {
        match i {
            0 => Ok(Turn::Left),
            1 => Ok(Turn::Right),
            _ => Err(PaintingError::InvalidTurn(i))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Facing {
    Up,
    Down,
    Left,
    Right
}

impl Facing {
    fn leftwards(&self) -> Facing {
        match self {
            Facing::Up => Facing::Left,
            Facing::Left => Facing::Down,
            Facing::Down => Facing::Right,
            Facing::Right => Facing::Up
        }
    }

    fn rightwards(&self) -> Facing {
        match self {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up
        }
    }

    fn turn(&self, turn: Turn) -> Facing {
        match turn {
            Turn::Left => self.leftwards(),
            Turn::Right => self.rightwards()
        }
    }

    fn step(&self, start: &Point) -> Point {
        let mut new_point = start.clone();
        match self {
            Facing::Up => new_point.y -= 1,
            Facing::Down => new_point.y += 1,
            Facing::Left => new_point.x -= 1,
            Facing::Right => new_point.x += 1
        };
        new_point
    }
}

#[derive(Debug)]
struct PainterBot {
    brain: Machine,
    canvas: HashMap<Point, Color>,
    location: Point,
    facing: Facing
}

impl PainterBot {
    fn new(machine: Machine) -> Self {
        PainterBot {
            brain: machine,
            canvas: HashMap::new(),
            location: Point { x: 0, y: 0 },
            facing: Facing::Up
        }
    }

    fn color_at(&self, point: &Point) -> &Color {
        self.canvas.get(point).or(Some(&Color::Black)).unwrap()
    }

    fn paint(&mut self, color: Color) {
        self.canvas.insert(self.location, color);
    }

    fn step(&mut self, turn: Turn) {
        self.facing = self.facing.turn(turn);
        self.location = self.facing.step(&self.location);
    }

    fn run(&mut self) -> PaintingResult<()> {
        loop {
            self.brain.run()?;

            match self.brain.state() {
                MachineState::Halted => { return Ok(()); },
                MachineState::Blocked => {
                    if self.brain.peek() >= 2 {
                        let output = self.brain.read();
                        self.paint(Color::from_int(output[0])?);
                        self.step(Turn::from_int(output[1])?);
                    }

                    self.brain.write(self.color_at(&self.location).to_int());
                },
                // Shouldn't ever get here after `run`.
                MachineState::Running => {
                    return Err(PaintingError::BrainStillRunning)
                }
            }
        }
    }

    fn canvas_str(&self) -> String {
        let mut painting = String::new();
        painting.push_str("\n");

        let min_x = self.canvas.keys().map(|p| p.x).min().unwrap();
        let min_y = self.canvas.keys().map(|p| p.y).min().unwrap();
        let max_x = self.canvas.keys().map(|p| p.x).max().unwrap();
        let max_y = self.canvas.keys().map(|p| p.y).max().unwrap();

        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                painting.push_str(&format!("{}", self.color_at(&Point { x: x, y: y })));
            }
            painting.push_str("\n");
        }

        painting
    }
}

pub struct DayEleven {}

impl Problem for DayEleven {
    fn part_one(&self, input: &str) -> String {
        let machine = Machine::from_str(input).unwrap();
        let mut painter = PainterBot::new(machine);

        painter.run().unwrap();

        painter.canvas.len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let machine = Machine::from_str(input).unwrap();
        let mut painter = PainterBot::new(machine);

        painter.paint(Color::White);
        painter.run().unwrap();

        painter.canvas_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
