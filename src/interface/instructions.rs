use std::{
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

use mlua::{Function, Lua};

use crate::{
    robot::{Robot, RobotCommand, RobotResponse},
    world::World,
};

pub fn simulate(
    lua: &Lua,
    mut robot: Robot,
    world: World,
    file: Vec<u8>,
) -> Result<Vec<RobotCommand>, Box<dyn std::error::Error>> {
    let globals = lua.globals();

    globals.set(
        "print",
        mlua::Value::Function(lua.create_function(|_, _v: String| Ok(())).unwrap()),
    )?;

    let (tx_in, rx_in) = channel::<RobotCommand>();
    let (tx_out, rx_out_raw) = channel::<RobotResponse>();

    let rx_out = Arc::new(Mutex::new(rx_out_raw));

    let tx_in_forward = tx_in.clone();
    let rx_out_forward = rx_out.clone();

    let tx_in_left = tx_in.clone();
    let rx_out_left = rx_out.clone();

    let tx_in_right = tx_in.clone();
    let rx_out_right = rx_out.clone();

    let tx_in_scan = tx_in.clone();
    let rx_out_scan = rx_out.clone();

    globals.set(
        "forward",
        mlua::Value::Function(lua.create_function(move |_, _: ()| {
            let _ = tx_in_forward.send(RobotCommand::Forward);

            match rx_out_forward.lock().unwrap().recv() {
                Ok(response) => match response {
                    RobotResponse::Ok => Ok(()),
                    _ => Err(mlua::Error::RuntimeError("Forward error".to_string())),
                },
                Err(_) => Ok(()),
            }
        })?),
    )?;

    globals.set(
        "left",
        mlua::Value::Function(lua.create_function(move |_, _: ()| {
            let _ = tx_in_left.send(RobotCommand::Left);

            // This cannot fail
            let _response = rx_out_left.lock().unwrap().recv();

            Ok(())
        })?),
    )?;

    globals.set(
        "right",
        mlua::Value::Function(lua.create_function(move |_, _: ()| {
            let _ = tx_in_right.send(RobotCommand::Right);

            // This cannot fail
            let _response = rx_out_right.lock().unwrap().recv();

            Ok(())
        })?),
    )?;

    globals.set(
        "scan",
        mlua::Value::Function(lua.create_function(move |_, _: ()| {
            let _ = tx_in_scan.send(RobotCommand::Scan);

            match rx_out_scan.lock().unwrap().recv().unwrap() {
                RobotResponse::Scan(state) => Ok(state),
                _ => Err(mlua::Error::RuntimeError("Scan error".to_string())),
            }
        })?),
    )?;

    let in_handle = thread::spawn(move || {
        let mut commands: Vec<RobotCommand> = vec![];
        const MAX_COMMANDS: usize = 2048;

        loop {
            if commands.len() >= MAX_COMMANDS {
                tx_out.send(RobotResponse::Error).unwrap();
                break;
            }

            let response = match rx_in.recv() {
                Ok(command) => {
                    commands.push(command);

                    match command {
                        RobotCommand::Forward => match robot.forward(&world) {
                            Ok(_) => RobotResponse::Ok,
                            Err(_) => RobotResponse::Error,
                        },
                        RobotCommand::Left => {
                            robot.left();
                            RobotResponse::Ok
                        }
                        RobotCommand::Right => {
                            robot.right();
                            RobotResponse::Ok
                        }
                        RobotCommand::Scan => RobotResponse::Scan(robot.scan(&world)),
                        RobotCommand::End => break,
                    }
                }
                Err(_) => break,
            };

            if robot.is_on_end_tile(&world) {
                tx_out.send(RobotResponse::Error).unwrap();

                break;
            }

            tx_out.send(response).unwrap();
        }

        commands
    });

    let chunk = lua.load(file);
    // TODO: Handle this error
    let main_function: Function = chunk.eval().unwrap();
    // TODO: Handle this error
    match main_function.call::<_, ()>(()) {
        Ok(_) => tx_in.send(RobotCommand::End).unwrap(),
        Err(_) => (),
    };

    Ok(in_handle.join().unwrap())
}
