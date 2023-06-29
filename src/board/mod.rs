use bevy::prelude::*;

use self::systems::{build_board, load_colors};

pub mod components;
pub mod resources;
pub mod systems;

pub const HEX_SIZE: f32 = 40.;
pub const HEX_GAP: f32 = 2.5;
pub const HEX_RADIUS: i32 = 5;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_colors.in_base_set(StartupSet::PreStartup))
            .add_startup_system(
                build_board
                    .in_base_set(StartupSet::Startup)
                    .after(load_colors),
            );
    }
}
