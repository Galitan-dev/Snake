extern crate bracket_lib;

use bracket_lib::prelude::*;
use game::UInt;

mod game;
pub mod palette;

pub const FPS_CAP: f32 = 40.0;
pub const COLUMNS: UInt = 80;
pub const ROWS: UInt = 50;

fn main() -> BError {
    palette::register();

    let context = BTermBuilder::simple80x50()
        .with_title("Snake")
        .with_fps_cap(FPS_CAP)
        .build()?;
    let gamestate = game::State::new();

    main_loop(context, gamestate)
}
