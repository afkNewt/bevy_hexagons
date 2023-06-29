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
        .add_plugin(BoardPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(UnitsPlugin)
        .add_plugin(TextPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
