extern crate bracket_lib;
extern crate console_error_panic_hook;

use bracket_lib::prelude::*;
use game::UInt;

mod game;
pub mod palette;

pub const FPS: f32 = 20.0;
pub const COLUMNS: UInt = 80;
pub const ROWS: UInt = 50;

fn main() -> BError {
    console_error_panic_hook::set_once();
    palette::register();

    let context = BTermBuilder::simple80x50()
        .with_title("Snake")
        .build()?;

    let gamestate = game::State::new();

    main_loop(context, gamestate)
}
