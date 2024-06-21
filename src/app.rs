use crate::robot::{Direction, Robot};
use crate::screens::game::GameScreen;
use crate::screens::{Screen, ScreenData};
use crate::textures::load_textures;
use crate::world::World;

const CELL_SIZE: i32 = 32;

pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let world_size: (u8, u8) = (11, 11);

    let mut world = World::new(world_size, (10, 5));

    for x in 0..world.width {
        world.set_tile((x, 0), crate::world::Tile::Wall);
        world.set_tile((x, world.height - 1), crate::world::Tile::Wall);
    }

    for y in 0..world.height {
        world.set_tile((0, y), crate::world::Tile::Wall);
        world.set_tile((world.width - 1, y), crate::world::Tile::Wall);
    }

    let robot_start_params = (1, 5, Direction::Right);

    let robot: Robot = Robot::new(
        robot_start_params.0,
        robot_start_params.1,
        robot_start_params.2,
    );

    let (mut rl, thread) = raylib::init()
        .size(
            (world_size.0 as i32 * CELL_SIZE) as i32,
            (world_size.1 as i32 * CELL_SIZE) as i32,
        )
        .title("Rust Navigator")
        .build();

    rl.set_target_fps(60);

    let textures = load_textures(&mut rl, &thread);

    let mut screen = GameScreen::new(world, robot, "./lua/test.lua");
    screen.initialize(ScreenData::new(&mut rl, &thread, &textures));

    while !screen.should_close() {
        screen.update();
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
