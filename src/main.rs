use bevy::prelude::*;
use board::BoardPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;

use tile_highlighting::TileHighlighting;
use units::UnitsPlugin;
use user_interface::UserInterfacePlugin;

pub mod hexagon;

pub mod board;
mod enemy;
mod player;
mod text;
mod tile_highlighting;
pub mod units;
mod user_interface;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(20, 20, 20)))
        .add_plugins(DefaultPlugins)
        .add_plugins((
            UserInterfacePlugin,
            BoardPlugin,
            EnemyPlugin,
            PlayerPlugin,
            UnitsPlugin,
            // TextPlugin,
            TileHighlighting,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
