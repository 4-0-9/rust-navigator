use std::collections::HashMap;

use raylib::{drawing::RaylibDrawHandle, text::Font, texture::Texture2D};

pub trait Drawable {
    fn draw(
        &self,
        position: (i32, i32),
        d: &mut RaylibDrawHandle,
        textures: &HashMap<String, Texture2D>,
        fonts: &HashMap<String, Font>,
    );
}
