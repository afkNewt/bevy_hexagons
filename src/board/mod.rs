use bevy::prelude::*;

use self::systems::{build_board, gizmo_spawner, load_colors};

pub mod components;
pub mod resources;
pub mod systems;

pub const HEX_SIZE: f32 = 40.;
pub const HEX_GAP: f32 = 2.5;
pub const HEX_RADIUS: i32 = 5;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_colors)
            .add_systems(Startup, build_board)
            .add_systems(Update, gizmo_spawner);
    }
}
