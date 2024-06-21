use raylib::prelude::*;

use interface::instructions::initialize_globals;
use std::{fs, sync::mpsc::channel};

use mlua::Function;
use std::thread;
use std::time::Duration;

use crate::robot::{Direction, Robot, RobotCommand, RobotError};
use crate::world::World;
use crate::{interface, robot};

const CELL_SIZE: u32 = 32;

pub fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let world_size: (u8, u8) = (9, 9);

    let world: World = World::new(world_size, (9, 4));
    let simulation_world = world.clone();

    let robot_start_params = (0, 4, Direction::Right);

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
                            Err(e) => {
                                break;
                            }
                        },
                        RobotCommand::Left => robot.left(),
                        RobotCommand::Right => robot.right(),
                        RobotCommand::Scan => {
                            println!("Sending scan result");
                            tx_response
                                .send(robot::RobotResponse::Scan(robot.scan(&simulation_world)))
                                .unwrap();
                        }
                        RobotCommand::End => break,
                    };
                }
                Err(_e) => (),
            };
        }

        commands
    });

    main_function.call::<_, ()>(())?;
    terminate_tx.send(RobotCommand::End).unwrap();

    let commands = simulation_handle.join().unwrap();

    let (mut rl, thread) = raylib::init()
        .size(
            (world_size.0 as u32 * CELL_SIZE + CELL_SIZE * 2).try_into()?,
            (world_size.1 as u32 * CELL_SIZE + CELL_SIZE * 2).try_into()?,
        )
        .title("Rust Navigator")
        .build();

    let mut command_index = 0;

    let mut paused = true;
    let mut playback_ended = false;

    rl.set_target_fps(60);
    let mut tick: u8 = 0;
    while !rl.window_should_close() && !(playback_ended && rl.is_key_down(KeyboardKey::KEY_SPACE)) {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if !paused {
            tick = (tick + 1) % 30;
            if !playback_ended && commands.len() > command_index && tick == 0 {
                match commands[command_index] {
                    RobotCommand::Forward => match robot.forward(&world) {
                        Ok(()) => (),
                        Err(_) => {
                            playback_ended = true;
                        }
                    },
                    RobotCommand::Left => robot.left(),
                    RobotCommand::Right => robot.right(),
                    RobotCommand::Scan => {
                        println!("Scan");
                    }
                    RobotCommand::End => playback_ended = true,
                };
                command_index += 1;
            }
        }

        draw_world(&mut d, &world);
        d.draw_rectangle(
            world.exit_position.0 as i32 * CELL_SIZE as i32 + CELL_SIZE as i32,
            world.exit_position.1 as i32 * CELL_SIZE as i32 + CELL_SIZE as i32,
            CELL_SIZE as i32,
            CELL_SIZE as i32,
            Color::WHITE,
        );
        draw_robot(&mut d, &robot);

        if playback_ended {
            d.draw_text("[SPACE] End", 4, 4, 24, Color::WHITE);
        } else {
            d.draw_text(
                match paused {
                    true => "[SPACE] Paused",
                    false => "[SPACE] Playing",
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

fn draw_world(d: &mut RaylibDrawHandle, world: &World) {
    for y in 0..world.height as i16 {
        for x in 0..world.width as i16 {
            let screen_pos = tile_to_screen_pos(x as u8, y as u8);

            match world.get_tile((x, y)) {
                Some(tile) => match tile {
                    crate::world::Tile::Empty => d.draw_rectangle(
                        screen_pos.0,
                        screen_pos.1,
                        CELL_SIZE as i32,
                        CELL_SIZE as i32,
                        Color::WHITE,
                    ),
                    crate::world::Tile::Wall => d.draw_rectangle(
                        screen_pos.0,
                        screen_pos.1,
                        CELL_SIZE as i32,
                        CELL_SIZE as i32,
                        Color::WHITE,
                    ),
                },
                None => (),
            }
        }
    }
}

// TODO: Show the robot's direction
fn draw_robot(d: &mut RaylibDrawHandle, robot: &Robot) {
    let screen_pos = tile_to_screen_pos_centered(robot.x, robot.y);

    d.draw_circle(
        screen_pos.0,
        screen_pos.1,
        CELL_SIZE as f32 / 2.0,
        Color::RED,
    );
}

fn tile_to_screen_pos(x: u8, y: u8) -> (i32, i32) {
    (
        (x as u32 * CELL_SIZE + CELL_SIZE) as i32,
        (y as u32 * CELL_SIZE + CELL_SIZE) as i32,
    )
}

fn tile_to_screen_pos_centered(x: u8, y: u8) -> (i32, i32) {
    let screen_pos = tile_to_screen_pos(x, y);
    (
        screen_pos.0 + (CELL_SIZE as f32 / 2.0).round() as i32,
        screen_pos.1 + (CELL_SIZE as f32 / 2.0).round() as i32,
    )
}
