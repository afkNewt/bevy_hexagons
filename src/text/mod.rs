use bevy::prelude::*;

use self::systems::{generate_tile_variant_text, update_tile_variant_text};

mod components;
pub mod systems;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_tile_variant_text)
            .add_system(update_tile_variant_text);
    }
}