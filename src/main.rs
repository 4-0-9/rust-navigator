use std::{fs, sync::mpsc::channel, thread, time::Duration};

use interface::instructions::initialize_globals;
use mlua::Function;
use rendering::renderer::{clear, render};
use robot::{Robot, RobotCommand, RobotError};
use world::World;

mod interface;
mod rendering;
pub mod robot;
pub mod world;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    clear();
    let world = World::new((9, 9), (9, 4));

    let mut robot = Robot::new(0, 4, robot::Direction::Right);

    render(&robot, &world);

    let (tx, rx) = channel::<RobotCommand>();
    let terminate_tx = tx.clone();

    let file = fs::read("./lua/test.lua")?;

    let lua = mlua::Lua::new();
    initialize_globals(&lua, tx)?;

    let chunk = lua.load(file);
    let main_function: Function = chunk.eval()?;

    let handle = thread::spawn(
        move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            loop {
                let command = rx.recv()?;

                match command {
                    RobotCommand::End => {
                        if robot.x as i16 != world.exit_position.0
                            || robot.y as i16 != world.exit_position.1
                        {
                            println!("You couldn't find the exit!");
                        }

                        break;
                    }
                    _ => thread::sleep(Duration::from_millis(500)),
                };

                match command {
                    RobotCommand::Forward => match robot.forward(&world) {
                        Ok(()) => (),
                        Err(e) => {
                            handle_error(e, &world);

                            break;
                        }
                    },
                    RobotCommand::Left => robot.left(),
                    RobotCommand::Right => robot.right(),
                    _ => {
                        dbg!("RobotCommand '{}' not implemented", command);
                        break;
                    },
                }

                clear();
                render(&robot, &world);
            }

            Ok(())
        },
    );

    main_function.call::<_, ()>(())?;

    terminate_tx.send(RobotCommand::End).unwrap();
    handle.join().unwrap().unwrap();

    Ok(())
}

fn handle_error(error: RobotError, world: &World) {
    match error {
        RobotError::InvalidMove(x, y) => {
            if x != world.exit_position.0 || y != world.exit_position.1 {
                println!("Oops! You hit an obstacle!");
            } else {
                println!("You reached the exit!");
            }
        }
    };
}
