use bevy::prelude::*;

use self::systems::{remove_tile_highlights, highlight_hovered_hex, highlight_unit_hex};

mod systems;

pub struct TileHighlighting;

impl Plugin for TileHighlighting {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (remove_tile_highlights, highlight_unit_hex, highlight_hovered_hex).chain());
    }
}