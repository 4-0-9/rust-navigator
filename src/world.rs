use raylib::prelude::*;

use std::collections::HashMap;

use crate::{app::tile_to_screen_pos, rendering::Drawable};

pub trait WorldTile {
    fn collision(&self) -> bool;
}

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Ground,
    Exit,
    Wall,
}

impl WorldTile for Tile {
    fn collision(&self) -> bool {
        match self {
            Tile::Ground => false,
            Tile::Exit => false,
            Tile::Wall => true,
        }
    }
}

impl Drawable for Tile {
    fn draw(
        &self,
        position: (i32, i32),
        d: &mut RaylibDrawHandle,
        textures: &HashMap<String, Texture2D>,
        _fonts: &HashMap<String, Font>,
    ) {
        d.draw_texture(
            match self {
                Tile::Ground => textures.get("ground").unwrap(),
                Tile::Exit => textures.get("exit").unwrap(),
                Tile::Wall => textures.get("wall").unwrap(),
            },
            position.0,
            position.1,
            Color::WHITE,
        );
    }
}

#[derive(Clone)]
pub struct World {
    pub width: u8,
    #[allow(dead_code)]
    pub height: u8,
    pub exit_position: (u8, u8),
    lock_exit_tile: bool,
    tiles: Vec<Tile>,
}

// TODO: Make the world include the border instead of this -1 / width / height shit
impl World {
    pub fn new(resolution: (u8, u8), exit_position: (u8, u8)) -> Self {
        let tiles = vec![Tile::Ground; resolution.0 as usize * resolution.1 as usize];

        let mut world = Self {
            width: resolution.0,
            height: resolution.1,
            exit_position,
            lock_exit_tile: false,
            tiles,
        };

        world.set_tile(exit_position, Tile::Exit);
        world.lock_exit_tile = true;

        world
    }

    fn get_tile_index(&self, position: (u8, u8)) -> usize {
        (position.1 * self.width + position.0) as usize
    }

    pub fn set_tile(&mut self, position: (u8, u8), tile: Tile) {
        if self.lock_exit_tile
            && position.0 == self.exit_position.0
            && position.1 == self.exit_position.1
        {
            return;
        }

        let index = self.get_tile_index(position);
        self.tiles[index] = tile;
    }

    pub fn get_tile(&self, position: (u8, u8)) -> Tile {
        let index = self.get_tile_index(position);

        self.tiles[index]
    }
}

impl Drawable for World {
    fn draw(
        &self,
        _position: (i32, i32),
        d: &mut RaylibDrawHandle,
        textures: &HashMap<String, Texture2D>,
        fonts: &HashMap<String, Font>,
    ) {
        for y in 0..self.height {
            for x in 0..self.width {
                let screen_pos = tile_to_screen_pos(x, y);

                self.get_tile((x, y)).draw(screen_pos, d, textures, fonts);
            }
        }
    }
}
