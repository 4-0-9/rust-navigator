use std::collections::HashMap;

use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};

pub mod editor;
pub mod game;
pub mod menu;

pub struct ScreenData<'a> {
    rl: &'a mut RaylibHandle,
    thread: &'a RaylibThread,
    textures: &'a HashMap<String, Texture2D>,
}

impl<'a> ScreenData<'a> {
    pub fn new(
        rl: &'a mut RaylibHandle,
        thread: &'a RaylibThread,
        textures: &'a HashMap<String, Texture2D>,
    ) -> Self {
        Self {
            rl,
            thread,
            textures,
        }
    }
}

pub trait Screen<'a> {
    fn initialize(&mut self, data: ScreenData<'a>);
    fn update(&mut self);
    fn should_close(&self) -> bool;
    fn end(self);
}
