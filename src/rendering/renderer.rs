use crate::{robot::Robot, world::World};

const EXIT_SYMBOL: &str = "🚪";
const BORDER_CHARACTER: &str = "🧱";

pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn render(robot: &Robot, world: &World) {
    print!("{}", BORDER_CHARACTER);
    for x in 0..world.height {
        if world.exit_position.1 != -1 || world.exit_position.0 != x.into() {
            print!("{BORDER_CHARACTER}");
        } else {
            print!("{}", EXIT_SYMBOL);
        }
    }
    print!("{BORDER_CHARACTER}\n");
    for y in 0..world.height {
        if world.exit_position.0 != -1 || world.exit_position.1 != y.into() {
            print!("{}", BORDER_CHARACTER);
        } else {
            print!("{}", EXIT_SYMBOL);
        }

        for x in 0..world.width {
            if robot.x != x || robot.y != y {
                let tile = world.get_tile((x, y)).unwrap();
                print!("{tile}");
            } else {
                print!("{robot}");
            }
        }

        if world.exit_position.0 != world.width.into() || world.exit_position.1 != y.into() {
            print!("{BORDER_CHARACTER}");
        } else {
            print!("{}", EXIT_SYMBOL);
        }

        print!("\n");
    }

    print!("{BORDER_CHARACTER}");

    for x in 0..world.height {
        if world.exit_position.1 != world.height.into() || world.exit_position.0 != x.into() {
            print!("{BORDER_CHARACTER}");
        } else {
            print!("{}", EXIT_SYMBOL);
        }
    }
    print!("{BORDER_CHARACTER}\n");
}
