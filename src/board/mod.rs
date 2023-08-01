use bevy::prelude::*;

use self::systems::{build_board, draw_borders, load_colors};

pub mod components;
pub mod resources;
pub mod systems;

// how many hex_rads larger the
// background hex should be
pub const BACKGROUND_HEX_SIZE: f32 = 1.8;

pub const HEX_SIZE: f32 = 40.;
pub const HEX_GAP: f32 = 2.5;
pub const HEX_RADIUS: i32 = 5;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_colors)
            .add_systems(Startup, build_board)
            .add_systems(Update, draw_borders);
    }
}
