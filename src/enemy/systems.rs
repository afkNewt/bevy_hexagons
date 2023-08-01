use bevy::prelude::*;

use crate::{
    board::components::{HexTile, TileVariant, Team},
    hexagon::Cube,
};

pub fn place_enemy_capital(mut hexes: Query<&mut HexTile>) {
    let enemy_capital = Cube::axial_new(0, 0);
    let enemy_tiles = enemy_capital.cube_neighbors();

    for mut hex in &mut hexes {
        if enemy_tiles.contains(&hex.coordinate) {
            hex.team = Team::Enemy;
        }

        if hex.coordinate == enemy_capital {
            hex.team = Team::Enemy;
            hex.variant = TileVariant::Capital;
        }
    }
}
