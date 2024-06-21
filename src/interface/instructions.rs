use std::sync::mpsc::{Receiver, Sender};

use mlua::Lua;

use crate::robot::{RobotCommand, RobotResponse};

pub fn initialize_globals(
    lua: &Lua,
    tx_commands: Sender<RobotCommand>,
    rx: Receiver<RobotResponse>,
) -> Result<(), Box<dyn std::error::Error>> {
    let globals = lua.globals();

    globals.set(
        "print",
        mlua::Value::Function(lua.create_function(|_, _v: String| Ok(())).unwrap()),
    )?;

    let forward_tx = tx_commands.clone();
    let left_tx = tx_commands.clone();
    let right_tx = tx_commands.clone();
    let scan_tx = tx_commands.clone();

    globals.set(
        "forward",
        mlua::Value::Function(lua.create_function(move |_, _: ()| {
            forward_tx.send(RobotCommand::Forward).unwrap();

            Ok(())
        })?),
    )?;

    globals.set(
        "left",
        mlua::Value::Function(lua.create_function(move |_, _: ()| {
            left_tx.send(RobotCommand::Left).unwrap();

            Ok(())
        })?),
    )?;

    globals.set(
        "right",
        mlua::Value::Function(lua.create_function(move |_, _: ()| {
            right_tx.send(RobotCommand::Right).unwrap();

            Ok(())
        })?),
    )?;

    globals.set(
        "scan",
        mlua::Value::Function(lua.create_function(move |_, _: ()| {
            scan_tx.send(RobotCommand::Scan).unwrap();

            match rx.recv() {
                Ok(response) => match response {
                    RobotResponse::Scan(state) => Ok(state),
                },
                Err(_) => panic!("Error scanning"),
            }
        })?),
    )?;

    Ok(())
}
