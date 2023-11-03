use std::fmt::Display;

use bracket_lib::terminal::{palette_color, register_palette_color, RGBA};

pub fn register() {
    register_palette_color("grey", (71, 106, 111));
    register_palette_color("green", (206, 247, 160));
    register_palette_color("pink", (255, 178, 230));
    register_palette_color("red", (239, 41, 23));
}

pub fn color<S: ToString + Display + Copy>(name: S) -> RGBA {
    palette_color(name).unwrap_or_else(|| panic!("Missing color: {name}"))
}
