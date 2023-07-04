use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::hexagon::{hexes_in_range, Cube};

use super::{
    components::{HexTile, TileVariant},
    resources::HexColors,
    HEX_GAP, HEX_RADIUS, HEX_SIZE,
};

pub fn load_colors(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(HexColors {
        backround_hex: materials.add(ColorMaterial::from(Color::rgb_u8(30, 30, 30))),
        neutral: materials.add(ColorMaterial::from(Color::rgb_u8(40, 40, 40))),
        neutral_hovered: materials.add(ColorMaterial::from(Color::rgb_u8(50, 50, 50))),
        ally_capital: materials.add(ColorMaterial::from(Color::rgb_u8(60, 60, 255))),
        ally_capital_hovered: materials.add(ColorMaterial::from(Color::rgb_u8(100, 100, 255))),
        enemy_capital: materials.add(ColorMaterial::from(Color::rgb_u8(255, 60, 60))),
        enemy_capital_hovered: materials.add(ColorMaterial::from(Color::rgb_u8(255, 100, 100))),
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

    let padded_size = HEX_SIZE + HEX_GAP;

    let hex_coords = hexes_in_range(HEX_RADIUS, Cube::axial_new(0, 0));
    for coord in hex_coords {
        // https://www.redblobgames.com/grids/hexagons/#hex-to-pixel
        let x = 3_f32.sqrt() * coord.q as f32 + 3_f32.sqrt() / 2. * coord.r as f32;
        let y = 3. / 2. * coord.r as f32;
        pointy_top_hex_mesh.transform.translation = Vec3::new(x * padded_size, y * padded_size, 1.);

        commands.spawn(pointy_top_hex_mesh.clone()).insert(HexTile {
            coordinate: coord,
            variant: TileVariant::Neutral,
            capture_progress: 0,
        });
    }

    // magic number is the x of vertex translation
    // with index 2 on a pointy hex
    let magic_number: f32 = (PI / 180. * 30.).cos();
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
