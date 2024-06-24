use std::collections::HashMap;

use raylib::text::Font;

use crate::screens::menu::MenuScreen;
use crate::screens::Screen;
use crate::textures::load_textures;

const CELL_SIZE: i32 = 32;
const TARGET_FPS: u32 = 120;

pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let (screen_width, screen_height) = (352.0, 352.0);

    let (mut rl, thread) = raylib::init()
        .size(352 as i32, 352 as i32)
        .title("Rust Navigator")
        .build();

    let mut fonts: HashMap<String, Font> = HashMap::new();
    fonts.insert(
        "geist".to_string(),
        rl.load_font_ex(&thread, "./fonts/GeistMono-Regular.ttf", 128, None)
            .unwrap(),
    );

    rl.set_target_fps(TARGET_FPS);

    let textures = load_textures(&mut rl, &thread);

    let mut screen: Box<dyn Screen> = Box::new(MenuScreen::new());
    screen.initialize(screen_width, screen_height, &textures, &fonts);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        if screen.update(&mut d, &textures, &fonts) {
            let new_screen = screen.get_new_screen();

            drop(screen);

            screen = new_screen;
            screen.initialize(screen_width, screen_height, &textures, &fonts);
        }
    }

    Ok(())
}

pub fn tile_to_screen_pos(x: u8, y: u8) -> (i32, i32) {
    ((x as i32 * CELL_SIZE) as i32, (y as i32 * CELL_SIZE) as i32)
}

pub fn tile_to_screen_pos_centered(x: u8, y: u8) -> (i32, i32) {
    let screen_pos = tile_to_screen_pos(x, y);
    (
        screen_pos.0 + (CELL_SIZE as f32 / 2.0).round() as i32,
        screen_pos.1 + (CELL_SIZE as f32 / 2.0).round() as i32,
    )
}
