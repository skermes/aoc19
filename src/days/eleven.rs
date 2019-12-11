use std::collections::HashMap;

use crate::problem::Problem;
use crate::intcode::{Machine,MachineState,OperationalError};
use thiserror::Error;

#[derive(Debug, Error)]
enum PaintingError {
    #[error("`{0}` is not a valid color.")]
    InvalidColor(isize),
    #[error("`{0}` is not a valid turn.")]
    InvalidTurn(isize),
    #[error("Error encountered while running intcode program.")]
    IntcodeError(#[from] OperationalError)
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

    fn run(&mut self) -> PaintingResult<()> {
        loop {
            self.brain.run()?;

            match self.brain.state() {
                MachineState::Halted => { return Ok(()); },
                MachineState::Blocked => {
                    self.brain.write(self.color_at(&self.location).to_int());
                },
                // Shouldn't ever get here after `run`.
                MachineState::Running => { }
            }
        }
    }
}

pub struct DayEleven {}

impl Problem for DayEleven {
    fn part_one(&self, input: &str) -> String {
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
