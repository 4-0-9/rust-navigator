use std::{collections::HashMap, fs, sync::mpsc::channel, thread};

use mlua::Function;
use raylib::prelude::*;

use crate::{
    app::tile_to_screen_pos_centered,
    interface::instructions::initialize_globals,
    rendering::Drawable,
    robot::{self, Robot, RobotCommand},
    world::World,
};

use super::Screen;

pub struct GameScreen {
    paused: bool,
    playback_ended: bool,
    tick: u8,
    robot: Robot,
    world: World,
    commands: Vec<RobotCommand>,
    command_index: usize,
}

impl GameScreen {
    pub fn new(world: World, mut robot: Robot, file_path: &str) -> GameScreen {
        let (tx_globals, rx_globals) = channel::<RobotCommand>();
        let (tx_response, rx_response) = channel();

        let terminate_tx = tx_globals.clone();

        // TODO: Handle this error
        let file = fs::read(file_path).unwrap();

        let lua = mlua::Lua::new();
        initialize_globals(&lua, tx_globals, rx_response).unwrap();

        let chunk = lua.load(file);
        // TODO: Handle this error
        let main_function: Function = chunk.eval().unwrap();

        let simulation_world = world.clone();

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

        // TODO: Handle this error
        main_function.call::<_, ()>(()).unwrap();
        terminate_tx.send(RobotCommand::End).unwrap();

        let commands = simulation_handle.join().unwrap();

        Self {
            paused: true,
            playback_ended: false,
            command_index: 0,
            tick: 0,
            robot,
            world,
            commands,
        }
    }
}
impl Screen for GameScreen {
    fn initialize(&mut self, _screen_width: f32, _screen_height: f32, _textures: &HashMap<String, Texture2D>, _fonts: &HashMap<String, Font>) {
    }

    fn update(&mut self, d: &mut RaylibDrawHandle, textures: &HashMap<String, Texture2D>, fonts: &HashMap<String, Font>) -> bool {
        if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
            self.paused = !self.paused;
        }

        d.clear_background(Color::BLACK);

        if !self.paused {
            self.tick = (self.tick + 1) % 30;

            if !self.playback_ended && self.commands.len() > self.command_index && self.tick == 0 {
                self.robot.scanning = false;
                match self.commands[self.command_index] {
                    RobotCommand::Forward => match self.robot.forward(&self.world) {
                        Ok(()) => (),
                        Err(_) => {
                            self.playback_ended = true;
                        }
                    },
                    RobotCommand::Left => self.robot.left(),
                    RobotCommand::Right => self.robot.right(),
                    RobotCommand::Scan => self.robot.scanning = true,
                    RobotCommand::End => self.playback_ended = true,
                };

                self.command_index += 1;

                if self.robot.is_on_end_tile(&self.world) {
                    self.playback_ended = true;
                }
            }
        }

        self.world.draw((0, 0), d, &textures, &fonts);
        self.robot.draw(
            tile_to_screen_pos_centered(self.robot.x, self.robot.y),
            d,
            textures,
            fonts,
        );

        if self.playback_ended {
            d.draw_text("[Escape] End", 4, 4, 24, Color::WHITE);
        } else {
            d.draw_text(
                match self.paused {
                    true => "[Space] Paused",
                    false => "[Space] Playing",
                },
                4,
                4,
                24,
                match self.paused {
                    true => Color::RED,
                    false => Color::GREEN,
                },
            );
        }

        false
    }

    fn get_new_screen(&self) -> Box<dyn Screen>
    {
        Box::new(GameScreen::new(
            World::new((0, 0), (1, 1)),
            Robot::new(0, 0, robot::Direction::Right),
            "",
        ))
    }
}
