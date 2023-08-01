use bevy::prelude::*;

const UNIT_SPRITE_SIZE: f32 = HEX_SIZE / 110.;

use crate::{
    board::{resources::HexColors, HEX_SIZE, components::Team},
    hexagon::{cursor_to_hex, hex_to_pixel, Cube},
};

use super::{
    components::{Action, Unit, UnitDefault},
    resources::SelectedUnit,
};

pub fn test_spawn_unit(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_unit(
        &mut commands,
        &asset_server,
        Cube::axial_new(-2, 4),
        UnitDefault::Knight,
        Team::Ally,
    );
    spawn_unit(
        &mut commands,
        &asset_server,
        Cube::axial_new(-2, 1),
        UnitDefault::Newt,
        Team::Enemy,
    );
    spawn_unit(
        &mut commands,
        &asset_server,
        Cube::axial_new(-3, 0),
        UnitDefault::Archer,
        Team::Enemy,
    );
}

fn spawn_unit(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    hex_pos: Cube,
    unit: UnitDefault,
    team: Team,
) {
    let (x, y) = hex_to_pixel(hex_pos);
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, 1.),
                scale: Vec3::splat(UNIT_SPRITE_SIZE),
                ..Default::default()
            },
            texture: asset_server.load(unit.sprite_location()),
            ..default()
        })
        .insert(Unit::new_default(unit, hex_pos, team));
}

pub fn check_for_unit_selection(
    windows: Query<&Window>,
    buttons: Res<Input<MouseButton>>,
    mut selected_unit: ResMut<SelectedUnit>,
    units: Query<(&Unit, Entity)>,
) {
    let Some(hovered_hex) = cursor_to_hex(windows) else {
        return;
    };

    if buttons.just_released(MouseButton::Right) {
        selected_unit.0 = None;
        return;
    }

    if !buttons.just_released(MouseButton::Left) {
        return;
    }

    for (unit, entity) in &units {
        if unit.position.q != hovered_hex.q {
            continue;
        }

        if unit.position.r != hovered_hex.r {
            continue;
        }

        selected_unit.0 = Some(entity);
        return;
    }

    selected_unit.0 = None;
}

pub fn despawn_dead_units(mut commands: Commands, units: Query<(Entity, &Unit)>) {
    for (entity, unit) in &units {
        if unit.health <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn check_for_unit_movement(
    windows: Query<&Window>,
    mut buttons: ResMut<Input<MouseButton>>,
    selected_unit: Res<SelectedUnit>,
    mut units: Query<(&mut Unit, &mut Transform, Entity)>,
) {
    // make sure we left clicked
    if !buttons.just_released(MouseButton::Left) {
        return;
    }

    // make sure we are hovering a hex
    let Some(hovered_hex) = cursor_to_hex(windows) else {
        return;
    };

    // make sure there is an entity selected
    let Some(selected_entity) = selected_unit.0 else {
        return;
    };

    // make sure the entity is a unit
    let Ok((unit, _unit_transform, _unit_id)) = units.get(selected_entity) else {
        return;
    };

    // make sure the entity is an ally
    if unit.team != Team::Ally {
        return;
    }

    if unit.absolute_attack_hexes().contains(&hovered_hex) && unit.actions.contains(&Action::Attack)
    {
        let (x, y) = hex_to_pixel(hovered_hex);

        let mut enemy_entity = None;
        for (enemy_unit, transform, entity) in &units {
            if enemy_unit.team == Team::Ally || transform.translation != Vec3::new(x, y, 1.0) {
                continue;
            }

            enemy_entity = Some(entity);
            break;
        }

        if let Some(enemy_entity) = enemy_entity {
            let entities = units.get_many_mut([selected_entity, enemy_entity]);
            if let Ok(
                [(mut attacker_unit, mut attacker_transform, _attacker_entity), (mut defender_unit, _defender_transform, _defender_entity)],
            ) = entities
            {
                attacker_unit.attack(&mut attacker_transform, &mut defender_unit);
                buttons.clear_just_released(MouseButton::Left);
                return;
            }
        }
    }

    let unit = units.get(selected_entity).unwrap().0;

    if unit.absolute_move_hexes().contains(&hovered_hex) && unit.actions.contains(&Action::Move) {
        // check if tile is occupied
        let (x, y) = hex_to_pixel(hovered_hex);
        for (_unit, transform, _entity) in &units {
            if transform.translation != Vec3::new(x, y, 1.0) {
                continue;
            }

            // tile is occupied
            return;
        }

        let (mut unit, mut unit_transform, _unit_id) = units.get_mut(selected_entity).unwrap();

        let (x, y) = hex_to_pixel(hovered_hex);
        unit_transform.translation = Vec3::new(x, y, 1.0);
        unit.position = hovered_hex;
        unit.remove_action(Action::Move);
    }
}

pub fn color_units(mut units: Query<(&Unit, &mut Sprite)>, colors: Res<HexColors>) {
    for (unit, mut sprite) in &mut units {
        match unit.team {
            Team::Ally => sprite.color = colors.ally_sprite,
            Team::Enemy => sprite.color = colors.enemy_sprite,
        }
    }
}
