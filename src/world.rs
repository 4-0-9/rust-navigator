#[derive(Clone, Copy, Debug)]
pub enum Tile {
    Empty,
    Wall,
}

#[derive(Clone)]
pub struct World {
    pub width: u8,
    #[allow(dead_code)]
    pub height: u8,
    pub exit_position: (i16, i16),
    tiles: Vec<Tile>,
}

// TODO: Make the world include the border instead of this -1 / width / height shit
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

    fn get_tile_index(&self, position: (i16, i16)) -> Option<usize> {
        if position.0 < 0
            || position.0 >= self.width.into()
            || position.1 < 0
            || position.1 >= self.height.into()
        {
            return None;
        }

        Some((position.1 * self.width as i16 + position.0) as usize)
    }

    pub fn set_tile(&mut self, position: (i16, i16), tile: Tile) {
        let index = self.get_tile_index(position).unwrap();
        self.tiles[index] = tile;
    }

    pub fn get_tile(&self, position: (i16, i16)) -> Option<Tile> {
        match self.get_tile_index(position) {
            Some(index) => self.tiles.get(index).copied(),
            None => None,
        }
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
