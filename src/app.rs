use raylib::prelude::*;

use interface::instructions::initialize_globals;
use std::{fs, sync::mpsc::channel};

use mlua::Function;
use std::thread;

use crate::rendering::Drawable;
use crate::robot::{Direction, Robot, RobotCommand};
use crate::textures::load_textures;
use crate::world::World;
use crate::{interface, robot};

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

    let simulation_world = world.clone();

    let robot_start_params = (1, 5, Direction::Right);

    let mut robot: Robot = Robot::new(
        robot_start_params.0,
        robot_start_params.1,
        robot_start_params.2,
    );

    let (tx_globals, rx_globals) = channel::<RobotCommand>();
    let (tx_response, rx_response) = channel();

    let terminate_tx = tx_globals.clone();

    let file = fs::read("./lua/test.lua")?;

    let lua = mlua::Lua::new();
    initialize_globals(&lua, tx_globals, rx_response)?;

    let chunk = lua.load(file);
    let main_function: Function = chunk.eval()?;

    let simulation_handle = thread::spawn(move || -> Vec<RobotCommand> {
        let mut commands: Vec<RobotCommand> = vec![];

        loop {
            let command = rx_globals.recv();

            match command {
                Ok(command) => {
                    commands.push(command);

                    match command {
                        RobotCommand::Forward => match robot.forward(&simulation_world) {
                            Ok(()) => (),
                            Err(_e) => {
                                break;
                            }
                        },
                        RobotCommand::Left => robot.left(),
                        RobotCommand::Right => robot.right(),
                        RobotCommand::Scan => {
                            tx_response
                                .send(robot::RobotResponse::Scan(robot.scan(&simulation_world)))
                                .unwrap();
                        }
                        RobotCommand::End => break,
                    };
                }
                Err(_e) => (),
            };

            if robot.is_on_end_tile(&simulation_world) {
                break;
            }
        }

        commands
    });

    main_function.call::<_, ()>(())?;
    terminate_tx.send(RobotCommand::End).unwrap();

    let commands = simulation_handle.join().unwrap();

    let (mut rl, thread) = raylib::init()
        .size(
            (world_size.0 as i32 * CELL_SIZE) as i32,
            (world_size.1 as i32 * CELL_SIZE) as i32,
        )
        .title("Rust Navigator")
        .build();

    let textures = load_textures(&mut rl, &thread);

    let mut tick: u8 = 0;
    let mut command_index = 0;
    let mut paused = true;
    let mut playback_ended = false;

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if !paused {
            tick = (tick + 1) % 30;

            if !playback_ended && commands.len() > command_index && tick == 0 {
                robot.scanning = false;
                match commands[command_index] {
                    RobotCommand::Forward => match robot.forward(&world) {
                        Ok(()) => (),
                        Err(_) => {
                            playback_ended = true;
                        }
                    },
                    RobotCommand::Left => robot.left(),
                    RobotCommand::Right => robot.right(),
                    RobotCommand::Scan => robot.scanning = true,
                    RobotCommand::End => playback_ended = true,
                };

                command_index += 1;

                if robot.is_on_end_tile(&world) {
                    playback_ended = true;
                }
            }
        }

        world.draw((0, 0), &mut d, &textures);
        robot.draw(
            tile_to_screen_pos_centered(robot.x, robot.y),
            &mut d,
            &textures,
        );

        if playback_ended {
            d.draw_text("[Escape] End", 4, 4, 24, Color::WHITE);
        } else {
            d.draw_text(
                match paused {
                    true => "[Space] Paused",
                    false => "[Space] Playing",
                },
                4,
                4,
                24,
                match paused {
                    true => Color::RED,
                    false => Color::GREEN,
                },
            );
        }
    }

    Ok(())
}

pub fn tile_to_screen_pos(x: u8, y: u8) -> (i32, i32) {
    ((x as i32 * CELL_SIZE) as i32, (y as i32 * CELL_SIZE) as i32)
}

fn tile_to_screen_pos_centered(x: u8, y: u8) -> (i32, i32) {
    let screen_pos = tile_to_screen_pos(x, y);
    (
        screen_pos.0 + (CELL_SIZE as f32 / 2.0).round() as i32,
        screen_pos.1 + (CELL_SIZE as f32 / 2.0).round() as i32,
    )
}
