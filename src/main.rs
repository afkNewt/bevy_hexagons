use bevy::prelude::*;
use board::BoardPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use text::TextPlugin;
use units::UnitsPlugin;

pub mod hexagon;

pub mod board;
mod enemy;
mod player;
mod text;
mod units;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(20, 20, 20)))
        .add_plugins(DefaultPlugins)
        .add_plugins((BoardPlugin, EnemyPlugin, PlayerPlugin, UnitsPlugin, TextPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}