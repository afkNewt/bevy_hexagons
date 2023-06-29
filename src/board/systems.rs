use std::{cmp, f32::consts::PI};

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

use crate::hexagon::Cube;

use super::{
    components::{HexTile, TileVariant},
    resources::HexColors,
    HEX_GAP, HEX_RADIUS, HEX_SIZE,
};

pub fn load_colors(mut materials: ResMut<Assets<ColorMaterial>>, mut colors: ResMut<HexColors>) {
    colors.ally_capital = materials.add(ColorMaterial::from(Color::rgb_u8(60, 60, 255)));
    colors.backround_hex = materials.add(ColorMaterial::from(Color::rgb_u8(30, 30, 30)));

    colors.neutral = materials.add(ColorMaterial::from(Color::rgb_u8(40, 40, 40)));
    colors.neutral_hovered = materials.add(ColorMaterial::from(Color::rgb_u8(50, 50, 50)));

    colors.ally_capital = materials.add(ColorMaterial::from(Color::rgb_u8(60, 60, 255)));
    colors.ally_capital_hovered = materials.add(ColorMaterial::from(Color::rgb_u8(100, 100, 255)));

    colors.enemy_capital = materials.add(ColorMaterial::from(Color::rgb_u8(255, 60, 60)));
    colors.enemy_capital_hovered = materials.add(ColorMaterial::from(Color::rgb_u8(255, 100, 100)));
}

pub fn build_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<HexColors>,
) {
    // vertex positions for a pointy topped hexagon
    // arranged like so
    //     1
    // 6       2
    //     0
    // 5       3
    //     4

    let mut v_pos = vec![[0., 0., 0.]];
    for i in 0..6 {
        let angle_deg = 60. * i as f32 - 30.;
        let angle_rad: f32 = PI / 180. * angle_deg;
        v_pos.push([angle_rad.cos(), angle_rad.sin(), 0.]);
    }

    let mut triangle = Mesh::new(PrimitiveTopology::TriangleList);
    triangle.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    triangle.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 0.1]; 7]);
    triangle.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 7]);
    triangle.set_indices(Some(Indices::U32(vec![
        0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4, 0, 6, 5, 0, 1, 6,
    ])));

    let mut pointy_top_hex_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(triangle).into(),
        material: colors.neutral.clone(),
        transform: Transform::from_scale(Vec3::new(HEX_SIZE, HEX_SIZE, 0.)),
        ..default()
    };

    // https://www.redblobgames.com/grids/hexagons/implementation.html#shape-hexagon
    let padded_size = HEX_SIZE + HEX_GAP;

    for q in -HEX_RADIUS..=HEX_RADIUS {
        let r1 = cmp::max(-HEX_RADIUS, -q - HEX_RADIUS);
        let r2 = cmp::min(HEX_RADIUS, -q + HEX_RADIUS);

        for r in r1..=r2 {
            // https://www.redblobgames.com/grids/hexagons/#hex-to-pixel
            let x = 3_f32.sqrt() * q as f32 + 3_f32.sqrt() / 2. * r as f32;
            let y = 3. / 2. * r as f32;
            pointy_top_hex_mesh.transform.translation =
                Vec3::new(x * padded_size, y * padded_size, 1.);

            commands.spawn(pointy_top_hex_mesh.clone()).insert(HexTile {
                coordinate: Cube::axial_new(q, r),
                variant: TileVariant::Neutral,
            });
        }
    }

    // vertex positions for a pointy topped hexagon
    // arranged like so
    //   1   2
    //
    // 6   0   3
    //
    //   5   4

    let mut v_pos = vec![[0., 0., 0.]];
    for i in 0..6 {
        let angle_deg = 60. * i as f32;
        let angle_rad = PI / 180. * angle_deg;
        v_pos.push([angle_rad.cos(), angle_rad.sin(), 0.]);
    }

    let mut triangle = Mesh::new(PrimitiveTopology::TriangleList);
    triangle.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    triangle.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 0.1]; 7]);
    triangle.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 7]);
    triangle.set_indices(Some(Indices::U32(vec![
        0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4, 0, 6, 5, 0, 1, 6,
    ])));

    // magic number is the x of vertex translation
    // with index 2 on a pointy hex
    let magic_number: f32 = (PI / 180. * 30.).cos();
    let scale = ((HEX_RADIUS as f32 * 2. + 1.8) * HEX_SIZE + HEX_RADIUS as f32 * 2. * HEX_GAP)
        * magic_number;

    let flat_top_hex_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(triangle).into(),
        material: colors.backround_hex.clone(),
        transform: Transform::from_scale(Vec3::new(scale, scale, 0.)),
        ..default()
    };

    commands.spawn(flat_top_hex_mesh);
}
