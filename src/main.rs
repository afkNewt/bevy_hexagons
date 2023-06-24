use bevy::prelude::*;
use board::BoardPlugin;
use player::PlayerPlugin;
use text::TextPlugin;

pub mod hexagon;

pub mod board;
mod player;
mod text;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(BoardPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TextPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}