use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::hexagon::{hex_to_pixel, hexes_in_range, Cube};

use super::{
    components::{HexTile, TileVariant},
    resources::HexColors,
    HEX_GAP, HEX_RADIUS, HEX_SIZE,
};

pub fn load_colors(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(HexColors {
        backround_hex: materials.add(ColorMaterial::from(Color::rgb_u8(25, 25, 25))),

        neutral: materials.add(ColorMaterial::from(Color::rgb_u8(40, 40, 40))),
        neutral_weak_highlight: materials.add(ColorMaterial::from(Color::rgb_u8(60, 60, 60))),
        neutral_strong_highlight: materials.add(ColorMaterial::from(Color::rgb_u8(90, 90, 90))),

        ally_capital: materials.add(ColorMaterial::from(Color::rgb_u8(70, 70, 200))),
        ally_capital_weak_highlight: materials.add(ColorMaterial::from(Color::rgb_u8(100, 100, 240))),
        ally_capital_strong_highlight: materials.add(ColorMaterial::from(Color::rgb_u8(150, 150, 255))),

        enemy_capital: materials.add(ColorMaterial::from(Color::rgb_u8(200, 70, 70))),
        enemy_capital_weak_highlight: materials.add(ColorMaterial::from(Color::rgb_u8(240, 100, 100))),
        enemy_capital_strong_highlight: materials.add(ColorMaterial::from(Color::rgb_u8(255, 150, 150))),
    });
}

pub fn build_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<HexColors>,
) {
    let mut pointy_top_hex_mesh = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(HEX_SIZE, 6).into())
            .into(),
        material: colors.neutral.clone(),
        ..default()
    };

    let hex_coords = hexes_in_range(HEX_RADIUS, Cube::axial_new(0, 0));
    for coord in hex_coords {
        // https://www.redblobgames.com/grids/hexagons/#hex-to-pixel
        let (x, y) = hex_to_pixel(coord);
        pointy_top_hex_mesh.transform.translation = Vec3::new(x, y, 1.);

        commands.spawn(pointy_top_hex_mesh.clone()).insert(HexTile {
            coordinate: coord,
            variant: TileVariant::Neutral,
            capture_progress: 0,
        });
    }

    let magic_number = (PI / 180. * 30.).cos();
    let scale = ((HEX_RADIUS as f32 * 2. + 1.8) * HEX_SIZE + HEX_RADIUS as f32 * 2. * HEX_GAP)
        * magic_number;

    let flat_top_hex_mesh = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(scale, 6).into())
            .into(),
        material: colors.backround_hex.clone(),
        transform: Transform::from_rotation(Quat::from_rotation_z(30_f32.to_radians())),
        ..default()
    };

    commands.spawn(flat_top_hex_mesh);
}
