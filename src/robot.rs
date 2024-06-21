use crate::world::World;

#[derive(Debug, Copy, Clone)]
pub enum RobotCommand {
    /// This command is sent by the program to tell the command receiver thread to shut down
    End,

    Forward,
    Left,
    Right,
    Scan,
}

#[derive(Debug)]
pub enum RobotResponse {
    Scan(bool),
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Robot {
    pub x: u8,
    pub y: u8,
    facing: Direction,
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

        match world.get_tile((self.x.into(), self.y.into())) {
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

    pub fn scan(&mut self, world: &World) -> bool {
        let forward_position: (i16, i16) = match self.facing {
            Direction::Left => (self.x as i16 - 1, self.y as i16),
            Direction::Right => (self.x as i16 + 1, self.y as i16),
            Direction::Up => (self.x as i16, self.y as i16 + 1),
            Direction::Down => (self.x as i16, self.y as i16 - 1),
        };

        match world.get_tile(forward_position) {
            Some(tile) => match tile {
                crate::world::Tile::Empty => false,
                crate::world::Tile::Wall => true,
            },
            None => true,
        }
    }
}
