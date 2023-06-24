use bevy::prelude::*;

use self::{systems::{load_colors, build_board}, resources::HexColors};

pub mod components;
pub mod resources;
mod systems;

pub const HEX_SIZE: f32 = 40.;
pub const HEX_GAP: f32 = 2.5;
pub const HEX_RADIUS: i32 = 5;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HexColors::default())
            .add_startup_system(load_colors)
            .add_startup_system(build_board);
    }
}