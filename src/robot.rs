use std::fmt::Display;

use crate::world::World;

#[derive(Debug)]
pub enum RobotCommand {
    /// This command is sent by the program to tell the command receiver thread to shut down
    End,

    Forward,
    Left,
    Right,
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub enum RobotError {
    InvalidMove(i16, i16),
}

type Result<T> = std::result::Result<T, RobotError>;

pub struct Robot {
    pub x: u8,
    pub y: u8,
    facing: Direction,
}

impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.facing {
            Direction::Left => "◀ ",
            Direction::Right => "▶ ",
            Direction::Up => "▲ ",
            Direction::Down => "▼ ",
        })
    }
}

impl Robot {
    pub fn new(x: u8, y: u8, orientation: Direction) -> Self {
        Self {
            x,
            y,
            facing: orientation,
        }
    }

    pub fn forward(&mut self, world: &World) -> Result<()> {
        match &self.facing {
            Direction::Left => {
                if self.x == 0 {
                    return Err(RobotError::InvalidMove(-1, self.y.into()));
                }

                self.x -= 1;
            }
            Direction::Right => {
                if self.x == world.width - 1 {
                    return Err(RobotError::InvalidMove(world.width.into(), self.y.into()));
                }

                self.x += 1;
            }
            Direction::Up => {
                if self.y == 0 {
                    return Err(RobotError::InvalidMove(self.x.into(), -1));
                }

                self.y -= 1;
            }
            Direction::Down => {
                if self.y == world.height - 1 {
                    return Err(RobotError::InvalidMove(self.x.into(), world.height.into()));
                }

                self.y += 1;
            }
        };

        match world.get_tile((self.x, self.y)) {
            Some(tile) => match tile {
                crate::world::Tile::Empty => Ok(()),
                crate::world::Tile::Wall => {
                    Err(RobotError::InvalidMove(self.x.into(), self.y.into()))
                }
            },
            None => Err(RobotError::InvalidMove(self.x.into(), self.y.into())),
        }
    }

    pub fn left(&mut self) {
        self.facing = match self.facing {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    }

    pub fn right(&mut self) {
        self.facing = match self.facing {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}
