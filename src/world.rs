use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Empty,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Empty => "  ",
            Self::Wall => "🧱",
        })
    }
}

pub struct World {
    pub width: u8,
    #[allow(dead_code)]
    pub height: u8,
    pub exit_position: (i16, i16),
    tiles: Vec<Tile>,
}

impl World {
    pub fn new(resolution: (u8, u8), exit_position: (i16, i16)) -> Self {
        let tiles = vec![Tile::Empty; resolution.0 as usize * resolution.1 as usize];

        Self {
            width: resolution.0,
            height: resolution.1,
            exit_position,
            tiles,
        }
    }

    fn get_tile_index(&self, position: (u8, u8)) -> usize {
        (position.1 * self.width + position.0).into()
    }

    pub fn set_tile(&mut self, position: (u8, u8), tile: Tile) {
        let index = self.get_tile_index(position);
        self.tiles[index] = tile;
    }

    pub fn get_tile(&self, position: (u8, u8)) -> Option<Tile> {
        let index = self.get_tile_index(position);
        self.tiles.get(index).copied()
    }
}

pub struct WorldIntoIterator {
    world: World,
    index: usize,
}

impl IntoIterator for World {
    type Item = Tile;
    type IntoIter = WorldIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        WorldIntoIterator {
            world: self,
            index: 0,
        }
    }
}

impl Iterator for WorldIntoIterator {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.index < self.world.tiles.len() {
            Some(self.world.tiles[self.index])
        } else {
            None
        };

        self.index += 1;

        result
    }
}
