#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Empty,
    Exit,
    Wall,
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
        let tiles = vec![Tile::Empty; resolution.0 as usize * resolution.1 as usize];

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
        if self.lock_exit_tile && position.0 == self.exit_position.0 && position.1 == self.exit_position.1 {
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
