use bevy::prelude::*;

use crate::{
    board::components::{HexTile, TileVariant, Team},
    hexagon::Cube,
};

pub fn place_enemy_capital(mut hexes: Query<&mut HexTile>) {
    let enemy_capital = Cube::axial_new(0, 0);
    let enemy_tiles = enemy_capital.cube_neighbors();

    for hex in &hexes {
        if (enemy_tiles.contains(&hex.coordinate) || hex.coordinate == enemy_capital)
            && hex.variant != TileVariant::Neutral
        {
            return;
        }
    }

    for mut hex in &mut hexes {
        if enemy_tiles.contains(&hex.coordinate) {
            hex.variant = TileVariant::Captured(Team::Enemy);
        }

        if hex.coordinate == enemy_capital {
            hex.variant = TileVariant::Capital(Team::Enemy);
        }
    }
}
