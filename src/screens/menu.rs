use std::collections::HashMap;

use raylib::prelude::*;

use crate::{components::button::Button, rendering::Drawable, robot::Robot, world::World};

use super::{game::GameScreen, Screen};

#[derive(Default)]
pub struct MenuScreen {
    width: f32,
    height: f32,
    play_button: Button,
}

impl MenuScreen {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Screen for MenuScreen {
    fn initialize(&mut self, screen_width: f32, screen_height: f32, _textures: &HashMap<String, Texture2D>, fonts: &HashMap<String, raylib::text::Font>) {
        self.width = screen_width;
        self.height = screen_height;

        let button_width = self.width / 2.0;

        // TODO: Remove this
        self.play_button = Button::new(
            Rectangle::new(
                self.width / 2.0 - button_width / 2.0,
                40.0,
                self.width as f32 / 2.0,
                40.0,
            ),
            "Play",
            Color::WHITESMOKE,
            Color::WHITE,
            Color::DARKGRAY,
            Color::GRAY,
            fonts.get("geist").unwrap(),
            24.0,
        );
    }

    fn update(&mut self, d: &mut RaylibDrawHandle, textures: &HashMap<String, Texture2D>, fonts: &HashMap<String, raylib::text::Font>) -> bool {
        d.clear_background(Color::BLACK);

        let mouse_clicked = d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);

        self.play_button.draw((0, 0), d, &textures, fonts);
        if mouse_clicked && self.play_button.is_hovered(&d) {
            return true;
        }

        false
    }

    fn get_new_screen(&self) -> Box<dyn Screen> {
        let mut world = World::new((11, 11), (10, 5));

        for x in 0..world.width {
            world.set_tile((x, 0), crate::world::Tile::Wall);
            world.set_tile((x, world.height - 1), crate::world::Tile::Wall);
        }

        for y in 0..world.height {
            world.set_tile((0, y), crate::world::Tile::Wall);
            world.set_tile((world.width - 1, y), crate::world::Tile::Wall);
        }

        let robot = Robot::new(1, 5, crate::robot::Direction::Right);

        Box::new(GameScreen::new(world, robot, "./lua/test.lua"))
    }
}
