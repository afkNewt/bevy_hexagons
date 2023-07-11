use bevy::prelude::*;

use self::{
    resources::{AllyCapital, PlayerCoins, TurnCounter},
    systems::{highlight_hovered_hex, pass_turn, place_ally_capital, remove_tile_highlights},
};

pub mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AllyCapital { position: None })
            .insert_resource(TurnCounter(0))
            .insert_resource(PlayerCoins(10))
            .add_systems(Update, (place_ally_capital, remove_tile_highlights, highlight_hovered_hex.after(remove_tile_highlights), pass_turn));
    }
}
