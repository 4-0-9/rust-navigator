use std::collections::HashMap;

use raylib::{drawing::RaylibDrawHandle, text::Font, texture::Texture2D};

pub mod editor;
pub mod game;
pub mod menu;

pub trait Screen {
    fn initialize(&mut self, screen_width: f32, screen_height: f32, textures: &HashMap<String, Texture2D>, fonts: &HashMap<String, Font>);
    // Returns whether the screen wants to end
    fn update(&mut self, d: &mut RaylibDrawHandle, textures: &HashMap<String, Texture2D>, fonts: &HashMap<String, raylib::text::Font>) -> bool;
    fn get_new_screen(&self) -> Box<dyn Screen>;
}
