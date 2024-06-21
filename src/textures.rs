use std::collections::HashMap;

use raylib::{texture::Texture2D, RaylibHandle, RaylibThread};

pub fn load_textures(rl: &mut RaylibHandle, thread: &RaylibThread) -> HashMap<String, Texture2D> {
    let rover = rl
        .load_texture(thread, "./assets/rover.png")
        .expect("Error loading rover");
    let ground = rl
        .load_texture(thread, "./assets/tiles/ground.png")
        .expect("Error loading ground");
    let wall = rl
        .load_texture(thread, "./assets/tiles/wall.png")
        .expect("Error loading wall");
    let exit = rl
        .load_texture(thread, "./assets/tiles/exit.png")
        .expect("Error loading exit");

    let mut textures: HashMap<String, Texture2D> = HashMap::with_capacity(4);

    textures.insert("rover".to_string(), rover);
    textures.insert("ground".to_string(), ground);
    textures.insert("wall".to_string(), wall);
    textures.insert("exit".to_string(), exit);

    textures
}
