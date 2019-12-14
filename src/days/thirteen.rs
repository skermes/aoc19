use std::collections::HashMap;
use std::fmt;
use std::io;
use std::str::FromStr;
use std::thread;
use std::time;

use thiserror::Error;
use itertools::Itertools;

use crate::problem::Problem;
use crate::intcode::{Machine,OperationalError,MachineState};

#[derive(Debug, Error)]
enum GameError {
    #[error("`{0}` is not a valid tile code.")]
    InvalidTile(isize),
    #[error("Error encountered while running game program: {0}")]
    ProgramError(#[from] OperationalError),
    #[error("Error reading from std in: {0}")]
    IoError(#[from] io::Error),
    #[error("`{0}` is not a valid joystick direction.")]
    InvalidJoystickDirection(String)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Tile::Empty => " ",
            Tile::Wall => "|",
            Tile::Block => "#",
            Tile::HorizontalPaddle => "=",
            Tile::Ball => "o"
        })
    }
}

impl Tile {
    fn from_int(i: isize) -> Result<Tile, GameError> {
        match i {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::HorizontalPaddle),
            4 => Ok(Tile::Ball),
            _ => Err(GameError::InvalidTile(i))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize
}

#[derive(Debug, PartialEq, Eq)]
enum JoystickDirection {
    Neutral,
    Left,
    Right
}

impl FromStr for JoystickDirection {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "h" => Ok(JoystickDirection::Left),
            "l" => Ok(JoystickDirection::Right),
            "" => Ok(JoystickDirection::Neutral),
            _ => Err(GameError::InvalidJoystickDirection(s.to_string()))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PlayMode {
    Interactive,
    Automatic,
    Invisible
}

#[derive(Debug)]
struct ArcadeGame {
    program: Machine,
    screen: HashMap<Point, Tile>,
    score: isize
}

const FPS: u64 = 15;

impl ArcadeGame {
    fn new(program: Machine) -> ArcadeGame {
        ArcadeGame {
            program: program,
            screen: HashMap::new(),
            score: 0
        }
    }

    fn tile_at(&self, point: Point) -> &Tile {
        self.screen.get(&point).or(Some(&Tile::Empty)).unwrap()
    }

    fn run(&mut self) -> Result<(), GameError> {
        self.program.run()?;
        Ok(())
    }

    fn process_output(&mut self) -> Result<(), GameError> {
        for chunk in &self.program.read().into_iter().chunks(3) {
            let (x, y, tile_id_or_score): (isize, isize, isize) = chunk
                .collect_tuple().unwrap();

            if x == -1 && y == 0 {
                self.score = tile_id_or_score;
            } else {
                self.screen.insert(
                    Point { x: x, y: y },
                    Tile::from_int(tile_id_or_score)?
                );
            }
        }

        Ok(())
    }

    fn move_joystick(&mut self, dir: JoystickDirection) {
        self.program.write(match dir {
            JoystickDirection::Neutral => 0,
            JoystickDirection::Left => -1,
            JoystickDirection::Right => 1
        });
    }

    fn play(&mut self, mode: PlayMode) -> Result<(), GameError> {
        loop {
            self.run()?;
            self.process_output()?;

            if mode != PlayMode::Invisible {
                println!("{}", self);
            }

            if self.program.state() == MachineState::Halted {
                return Ok(());
            }

            if mode == PlayMode::Interactive {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                self.move_joystick(input.trim().parse()?);
            } else {
                let (paddle_pos, _) = self.screen.iter()
                    .filter(|(point, tile)| **tile == Tile::HorizontalPaddle)
                    .nth(0).unwrap();
                let (ball_pos, _) = self.screen.iter()
                    .filter(|(point, tile)| **tile == Tile::Ball)
                    .nth(0).unwrap();

                // From watching this play, it doesn't seem to be a super
                // efficient strategy, but it works.
                if paddle_pos.x < ball_pos.x {
                    self.move_joystick(JoystickDirection::Right);
                } else if paddle_pos.x > ball_pos.x {
                    self.move_joystick(JoystickDirection::Left);
                } else {
                    self.move_joystick(JoystickDirection::Neutral);
                }
            }

            if mode == PlayMode::Automatic {
                thread::sleep(time::Duration::from_millis(1000 / FPS));
            }
        }
    }
}

impl fmt::Display for ArcadeGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Score: {}\n\n", self.score)?;

        if self.screen.len() > 0 {
            let min_x = self.screen.keys().map(|p| p.x).min().unwrap();
            let min_y = self.screen.keys().map(|p| p.y).min().unwrap();
            let max_x = self.screen.keys().map(|p| p.x).max().unwrap();
            let max_y = self.screen.keys().map(|p| p.y).max().unwrap();


            for y in min_y..max_y + 1 {
                for x in min_x..max_x + 1 {
                    write!(f, "{}", self.tile_at(Point { x: x, y: y }))?;
                }
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

pub struct DayThirteen {}

impl Problem for DayThirteen {
    fn name(&self) -> String {
        "Care Package".to_string()
    }

    fn part_one(&self, input: &str) -> String {
        let mut game = ArcadeGame::new(Machine::from_str(input).unwrap());
        game.run().unwrap();
        game.process_output().unwrap();

        game.screen.values()
            .filter(|tile| **tile == Tile::Block)
            .count()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut game = ArcadeGame::new(Machine::from_str(input).unwrap());
        game.program.set(0, 2).unwrap();
        game.play(PlayMode::Invisible).unwrap();

        game.score.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
