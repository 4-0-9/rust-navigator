use crate::{
    rendering::Drawable,
    world::{World, WorldTile},
};
use raylib::prelude::*;

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
    Ok,
    Error,
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
    InvalidMove(u8, u8),
}

type Result<T> = std::result::Result<T, RobotError>;

#[derive(Clone, Copy)]
pub struct Robot {
    pub x: u8,
    pub y: u8,
    pub scanning: bool,
    facing: Direction,
}

impl Robot {
    pub fn new(x: u8, y: u8, orientation: Direction) -> Self {
        Self {
            x,
            y,
            scanning: false,
            facing: orientation,
        }
    }

    pub fn forward(&mut self, world: &World) -> Result<()> {
        let forward_position = self.get_forward_position();

        if world.get_tile(forward_position).collision() {
            return Err(RobotError::InvalidMove(
                forward_position.0,
                forward_position.1,
            ));
        }

        self.x = forward_position.0;
        self.y = forward_position.1;

        Ok(())
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

    pub fn get_forward_position(&self) -> (u8, u8) {
        match self.facing {
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
        }
    }

    pub fn scan(&mut self, world: &World) -> bool {
        world.get_tile(self.get_forward_position()).collision()
    }

    pub fn is_on_end_tile(&self, world: &World) -> bool {
        self.x == world.exit_position.0 && self.y == world.exit_position.1
    }

    pub fn get_draw_rotation(&self) -> f32 {
        match self.facing {
            Direction::Left => 180.0,
            Direction::Right => 0.0,
            Direction::Up => 270.0,
            Direction::Down => 90.0,
        }
    }
}

impl Drawable for Robot {
    fn draw(
        &self,
        position: (i32, i32),
        d: &mut raylib::prelude::RaylibDrawHandle,
        textures: &std::collections::HashMap<String, raylib::texture::Texture2D>,
        _fonts: &std::collections::HashMap<String, Font>,
    ) {
        d.draw_texture_pro(
            textures.get("rover").unwrap(),
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: 32.0,
                height: 32.0,
            },
            Rectangle {
                x: position.0 as f32,
                y: position.1 as f32,
                width: 32.0,
                height: 32.0,
            },
            Vector2 { x: 16.0, y: 16.0 },
            self.get_draw_rotation(),
            match self.scanning {
                true => Color::REBECCAPURPLE,
                false => Color::WHITE,
            },
        );
    }
}
