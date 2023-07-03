use bevy::prelude::*;

use crate::{
    board::{
        components::{HexTile, TileVariant},
        resources::HexColors,
    },
    hexagon::{cube_distance, cursor_to_hex, Cube},
    units::components::Unit,
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
    mut units: Query<&mut Unit>,
    hex_tiles: Query<&mut HexTile>,
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

    for mut unit in &mut units {
        unit.new_turn();
    }

    // give the player a garanteed
    // 2 coins per turn
    player_coin_count.0 += 2;

    turn_counter.0 += 1;

    update_capture_progress(hex_tiles, units.to_readonly());
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

fn update_capture_progress(mut tiles: Query<&mut HexTile>, units: Query<&Unit>) {
    let ally_capital = tiles.iter().find(|t| t.variant == TileVariant::AllyCapital);
    let enemy_capital = tiles
        .iter()
        .find(|t| t.variant == TileVariant::EnemyCapital);

    if ally_capital.is_none() || enemy_capital.is_none() {
        return;
    }

    let ally_capital_pos = ally_capital.unwrap().coordinate;
    let enemy_capital_pos = enemy_capital.unwrap().coordinate;

    let ally_tiles: Vec<Cube> = tiles
        .iter()
        .filter(|t| t.variant == TileVariant::AllyLand || t.variant == TileVariant::AllyCapital)
        .map(|t| t.coordinate)
        .collect();

    let enemy_tiles: Vec<Cube> = tiles
        .iter()
        .filter(|t| t.variant == TileVariant::EnemyLand || t.variant == TileVariant::EnemyCapital)
        .map(|t| t.coordinate)
        .collect();

    let unit_vec: Vec<(Cube, bool)> = units.iter().map(|u| (u.position, u.ally)).collect();

    for mut tile in &mut tiles {
        let mut progress = 0;

        if unit_vec.contains(&(tile.coordinate, false))
            && (tile.variant != TileVariant::EnemyCapital || tile.variant != TileVariant::EnemyLand)
        {
            let mut can_progress = false;
            for neighbor in tile.coordinate.cube_neighbors() {
                if enemy_tiles.contains(&neighbor) {
                    can_progress = true;
                    break;
                }
            }

            if can_progress {
                progress = -1;
            }
        }

        if unit_vec.contains(&(tile.coordinate, true))
            && (tile.variant != TileVariant::AllyCapital || tile.variant != TileVariant::AllyLand)
        {
            let mut can_progress = false;
            for neighbor in tile.coordinate.cube_neighbors() {
                if ally_tiles.contains(&neighbor) {
                    can_progress = true;
                    break;
                }
            }

            if can_progress {
                progress = 1;
            }
        }

        tile.capture_progress += progress;
    }

    for mut tile in &mut tiles {
        if unit_vec.contains(&(tile.coordinate, false))
            || unit_vec.contains(&(tile.coordinate, true))
        {
            continue;
        }

        match tile.variant {
            TileVariant::Neutral => {
                if tile.capture_progress == 0 {
                    continue;
                }

                tile.capture_progress =
                    (tile.capture_progress.abs() - 1) * tile.capture_progress.signum();
            }
            TileVariant::AllyLand | TileVariant::AllyCapital => {
                tile.capture_progress = (tile.capture_progress + 1).min(capture_time(
                    cube_distance(ally_capital_pos, tile.coordinate),
                ));
            }
            TileVariant::EnemyLand | TileVariant::EnemyCapital => {
                tile.capture_progress = (tile.capture_progress - 1).max(-capture_time(
                    cube_distance(enemy_capital_pos, tile.coordinate),
                ));
            }
        }
    }

    for mut tile in &mut tiles {
        match tile.capture_progress.signum() {
            1 => {
                let capture_progress =
                    check_for_capture(ally_tiles.clone(), ally_capital_pos, &tile);
                let Some(capture_progress) = capture_progress else {
                    continue;
                };

                if tile.variant == TileVariant::AllyCapital {
                    continue;
                }

                if tile.variant == TileVariant::EnemyCapital {
                    tile.variant = TileVariant::AllyCapital;
                    println!("Allies Won!");
                } else {
                    tile.variant = TileVariant::AllyLand;
                }

                tile.capture_progress = capture_progress;
            }
            -1 => {
                let capture_progress =
                    check_for_capture(enemy_tiles.clone(), enemy_capital_pos, &tile);
                let Some(capture_progress) = capture_progress else {
                    continue;
                };

                if tile.variant == TileVariant::EnemyCapital {
                    continue;
                }

                if tile.variant == TileVariant::AllyCapital {
                    tile.variant = TileVariant::EnemyCapital;
                    println!("Enemies Won");
                } else {
                    tile.variant = TileVariant::EnemyLand;
                }

                tile.capture_progress = -capture_progress;
            }
            _ => {}
        }
    }
}

fn capture_time(steps_from_capital: i32) -> i32 {
    const MIN_CAPTURE_TIME: i32 = 2;
    const STEPS_BETWEEN_INCREMENT: i32 = 2;

    let capture_time = (steps_from_capital / STEPS_BETWEEN_INCREMENT) + 1;

    capture_time.max(MIN_CAPTURE_TIME)
}

fn check_for_capture(captured_tiles: Vec<Cube>, capital_pos: Cube, tile: &HexTile) -> Option<i32> {
    let mut can_capture = false;
    for neighbor in tile.coordinate.cube_neighbors() {
        if captured_tiles.contains(&neighbor) {
            can_capture = true;
            break;
        }
    }

    if !can_capture {
        return None;
    }

    let capture_threshold = capture_time(cube_distance(capital_pos, tile.coordinate));
    if tile.capture_progress.abs() >= capture_threshold {
        return Some(capture_time(cube_distance(capital_pos, tile.coordinate)));
    }

    None
}
