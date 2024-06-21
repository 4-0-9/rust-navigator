use std::collections::HashMap;

use raylib::{drawing::RaylibDrawHandle, texture::Texture2D};

pub trait Drawable {
    fn draw(&self, position: (i32, i32), d: &mut RaylibDrawHandle, textures: &HashMap<String, Texture2D>);
}
