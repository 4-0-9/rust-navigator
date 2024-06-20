use std::sync::mpsc::Sender;

use mlua::Lua;

use crate::robot::RobotCommand;

pub fn initialize_globals(lua: &Lua, tx: Sender<RobotCommand>) -> Result<(), mlua::Error> {
    let globals = lua.globals();

    globals.set(
        "print",
        mlua::Value::Function(lua.create_function(|_, _v: String| Ok(())).unwrap()),
    )?;

    let forward_tx = tx.clone();
    let left_tx = tx.clone();
    let right_tx = tx.clone();

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

    Ok(())
}
