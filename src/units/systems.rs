use bevy::prelude::*;

use crate::{
    board::{
        components::{HexTile, TileVariant},
        resources::HexColors,
    },
    hexagon::{cursor_to_hex, hex_to_pixel, pixel_to_hex, Cube},
};

use super::{components::Unit, resources::SelectedUnit};

pub fn test_spawn_unit(mut commands: Commands, asset_server: Res<AssetServer>) {
    let (x, y) = hex_to_pixel(Cube::axial_new(-2, 4));
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, 1.),
                scale: Vec3::splat(0.075),
                ..Default::default()
            },
            texture: asset_server.load("sprites/kl.png"),
            ..default()
        })
        .insert(Unit::test_new(Cube::axial_new(-2, 4)));
}

pub fn check_for_unit_selection(
    windows: Query<&Window>,
    buttons: Res<Input<MouseButton>>,
    mut selected_unit: ResMut<SelectedUnit>,
    units: Query<&Unit>,
) {
    let Some(hovered_hex) = cursor_to_hex(windows) else {
        return;
    };

    if selected_unit.0.is_some() && buttons.just_released(MouseButton::Right) {
        selected_unit.0 = None;
        return;
    }

    if !buttons.just_released(MouseButton::Left) {
        return;
    }

    for unit in &units {
        if unit.position.q != hovered_hex.q {
            continue;
        }

        if unit.position.r != hovered_hex.r {
            continue;
        }

        selected_unit.0 = Some(unit.clone());
        return;
    }

    selected_unit.0 = None;
}

pub fn highlight_unit_hex(
    selected_unit: Res<SelectedUnit>,
    mut hexes: Query<(&HexTile, &mut Handle<ColorMaterial>)>,
    colors: Res<HexColors>,
) {
    let Some(selected_unit) = selected_unit.0.clone() else {
        return;
    };

    let attack = selected_unit.absolute_attack_hexes();
    let movement = selected_unit.absolute_move_hexes();

    for (hex, mut color_mat) in &mut hexes {
        *color_mat = match hex.variant {
            TileVariant::Neutral => colors.neutral.clone(),
            TileVariant::AllyCapital => colors.ally_capital.clone(),
            TileVariant::EnemyCapital => colors.enemy_capital.clone(),
            _ => colors.neutral.clone(),
        };

        if !attack.contains(&hex.coordinate) && !movement.contains(&hex.coordinate) {
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
