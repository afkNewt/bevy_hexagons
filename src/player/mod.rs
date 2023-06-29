use bevy::prelude::*;

use self::{systems::{place_ally_capital, highlight_hovered_hex}, resources::AllyCapital};

mod resources;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AllyCapital { position: None })
            .add_systems((
            place_ally_capital,
            highlight_hovered_hex
        ));
    }
}