use bevy::prelude::*;

use crate::{board::{components::HexTile, resources::HexColors}, hexagon::cursor_to_hex, units::{components::{Unit, Action}, resources::SelectedUnit}};

pub fn remove_tile_highlights(
    mut hexes: Query<(&HexTile, &mut Handle<ColorMaterial>)>,
    colors: Res<HexColors>,
) {
    for (hex, mut color_mat) in &mut hexes {

        *color_mat = hex.base_color(&colors);
    }
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
        if hex.coordinate.q != hovered_hex.q {
            continue;
        }

        if hex.coordinate.r != hovered_hex.r {
            continue;
        }

        *color_mat = hex.strong_highlight(&colors);
        return;
    }
}


pub fn highlight_unit_hex(
    selected_unit: Res<SelectedUnit>,
    units: Query<&Unit>,
    mut hexes: Query<(&HexTile, &mut Handle<ColorMaterial>)>,
    colors: Res<HexColors>,
) {
    let Some(selected_entity) = selected_unit.0 else {
        return;
    };

    let Ok(unit) = units.get(selected_entity) else {
        return;
    };

    let mut strong_highlights = Vec::new();
    let mut weak_highlights = Vec::new();

    if unit.actions.contains(&Action::Attack) {
        strong_highlights.append(&mut unit.absolute_attack_hexes());
    } else {
        weak_highlights.append(&mut unit.absolute_attack_hexes());
    };

    if unit.actions.contains(&Action::Move) {
        strong_highlights.append(&mut unit.absolute_move_hexes());
    } else {
        weak_highlights.append(&mut unit.absolute_move_hexes());
    };

    for (hex, mut color_mat) in &mut hexes {
        if strong_highlights.contains(&hex.coordinate){
            *color_mat = hex.strong_highlight(&colors);
            continue;
        }

        if weak_highlights.contains(&hex.coordinate){
            *color_mat = hex.weak_highlight(&colors);
            continue;
        }
    }
}