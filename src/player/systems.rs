use bevy::prelude::*;

use crate::{
    board::{
        components::{HexTile, TileVariant},
        resources::HexColors,
    },
    hexagon::cursor_to_hex,
};

use super::resources::{AllyCapital, PlayerCoins, TurnCounter};

pub fn place_ally_capital(
    mut ally_capital: ResMut<AllyCapital>,
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    mut hexes: Query<&mut HexTile>,
) {
    if ally_capital.position.is_some() {
        return;
    }
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(hovered_hex) = cursor_to_hex(windows) else { return; };

    let claim_tiles = hovered_hex.cube_neighbors();

    for hex in &hexes {
        if (claim_tiles.contains(&hex.coordinate) || hex.coordinate == hovered_hex)
            && hex.variant != TileVariant::Neutral
        {
            return;
        }
    }

    for mut hex in &mut hexes {
        if claim_tiles.contains(&hex.coordinate) {
            hex.variant = TileVariant::AllyLand;
        }

        if hex.coordinate == hovered_hex {
            hex.variant = TileVariant::AllyCapital;
            ally_capital.position = Some(hovered_hex);
        }
    }
}

pub fn pass_turn(
    mut turn_counter: ResMut<TurnCounter>,
    mut player_coin_count: ResMut<PlayerCoins>,
    hex_tiles: Query<&HexTile>,
    keys: Res<Input<KeyCode>>,
) {
    if !keys.just_released(KeyCode::Space) {
        return;
    }

    for hex_tile in &hex_tiles {
        if hex_tile.variant == TileVariant::AllyLand {
            player_coin_count.0 += 1;
        }
    }

    // give the player a garanteed
    // 2 coins per turn
    player_coin_count.0 += 2;

    turn_counter.0 += 1;
}

pub fn highlight_hovered_hex(
    windows: Query<&Window>,
    mut hexes: Query<(&HexTile, &mut Handle<ColorMaterial>)>,
    colors: Res<HexColors>,
) {
    let Some(hovered_hex) = cursor_to_hex(windows) else {
        return;
    };

    for (hex, mut color_mat) in &mut hexes {
        *color_mat = match hex.variant {
            TileVariant::Neutral => colors.neutral.clone(),
            TileVariant::AllyCapital => colors.ally_capital.clone(),
            TileVariant::EnemyCapital => colors.enemy_capital.clone(),
            _ => colors.neutral.clone(),
        };

        if hex.coordinate.q != hovered_hex.q {
            continue;
        }

        if hex.coordinate.r != hovered_hex.r {
            continue;
        }

        *color_mat = match hex.variant {
            TileVariant::Neutral => colors.neutral_hovered.clone(),
            TileVariant::AllyCapital => colors.ally_capital_hovered.clone(),
            TileVariant::EnemyCapital => colors.enemy_capital_hovered.clone(),
            _ => colors.neutral_hovered.clone(),
        };
    }
}
