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
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugins(BoardPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(UnitsPlugin)
        .add_plugins(TextPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}