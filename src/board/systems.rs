use bevy::{
    prelude::*, sprite::MaterialMesh2dBundle,
};

use crate::hexagon::{hex_to_pixel, hexes_in_range, Cube};

use super::{
    components::{HexTile, TileVariant},
    resources::HexColors,
    HEX_RADIUS, HEX_SIZE,
};

pub fn load_colors(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(HexColors {
        backround_hex: materials.add(ColorMaterial::from(Color::rgb_u8(25, 25, 25))),

        neutral: materials.add(ColorMaterial::from(Color::rgb_u8(40, 40, 40))),
        neutral_weak_highlight: materials.add(ColorMaterial::from(Color::rgb_u8(60, 60, 60))),
        neutral_strong_highlight: materials.add(ColorMaterial::from(Color::rgb_u8(90, 90, 90))),

        ally_sprite: Color::rgb_u8(70, 130, 250),
        ally_capital: materials.add(ColorMaterial::from(Color::rgb_u8(70, 70, 200))),
        ally_capital_weak_highlight: materials
            .add(ColorMaterial::from(Color::rgb_u8(100, 100, 240))),
        ally_capital_strong_highlight: materials
            .add(ColorMaterial::from(Color::rgb_u8(150, 150, 255))),

        enemy_sprite: Color::rgb_u8(250, 130, 70),
        enemy_capital: materials.add(ColorMaterial::from(Color::rgb_u8(200, 70, 70))),
        enemy_capital_weak_highlight: materials
            .add(ColorMaterial::from(Color::rgb_u8(240, 100, 100))),
        enemy_capital_strong_highlight: materials
            .add(ColorMaterial::from(Color::rgb_u8(255, 150, 150))),
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
        pointy_top_hex_mesh.transform.translation = Vec3::new(x, y, 0.);

        commands.spawn(pointy_top_hex_mesh.clone()).insert(HexTile {
            coordinate: coord,
            variant: TileVariant::Neutral,
            capture_progress: 0,
        });
    }

    // let magic_number = (PI / 180. * 30.).cos();
    // let scale = ((HEX_RADIUS as f32 * 2. + 1.8) * HEX_SIZE + HEX_RADIUS as f32 * 2. * HEX_GAP)
    //     * magic_number;

    // let flat_top_hex_mesh = MaterialMesh2dBundle {
    //     mesh: meshes
    //         .add(shape::RegularPolygon::new(scale, 6).into())
    //         .into(),
    //     material: colors.backround_hex.clone(),
    //     transform: Transform::from_rotation(Quat::from_rotation_z(30_f32.to_radians())),
    //     ..default()
    // };

    // commands.spawn(flat_top_hex_mesh);
}

pub fn gizmo_spawner(mut gizmos: Gizmos, hexes: Query<&HexTile>) {
    let point_group = tile_border(hexes, vec![TileVariant::AllyLand, TileVariant::AllyCapital]);

    let Some(point_group) = point_group else {
        return;
    };

    for points in point_group {
        gizmos.linestrip_2d(points.clone(), Color::RED);

        for position in points {
            gizmos.circle(
                Vec3::new(position.x, position.y, 1.0),
                Vec3::Z,
                10.,
                Color::BLUE,
            );
        }
    }
}

fn tile_border(hexes: Query<&HexTile>, variants: Vec<TileVariant>) -> Option<Vec<Vec<Vec2>>> {
    let valid_tiles = hexes_in_range(HEX_RADIUS, Cube::axial_new(0, 0));

    let mut unsorted_points = hexes
        .iter()
        .filter(|h| variants.contains(&h.variant))
        .flat_map(|h| {
            let neighbor_coords = h.coordinate.cube_neighbors();
            let (h_x, h_y) = hex_to_pixel(h.coordinate);

            [
                neighbor_coords
                    .iter()
                    .filter(|c| !valid_tiles.contains(c))
                    .map(|c| {
                        let (c_x, c_y) = hex_to_pixel(*c);
                        Vec2::new((c_x + h_x) / 2., (c_y + h_y) / 2.)
                    })
                    .collect::<Vec<_>>(),
                hexes
                    .iter()
                    .filter(|h| neighbor_coords.contains(&h.coordinate))
                    .filter_map(|n| {
                        if !variants.contains(&n.variant) {
                            let (n_x, n_y) = hex_to_pixel(n.coordinate);
                            return Some(Vec2::new((n_x + h_x) / 2., (n_y + h_y) / 2.));
                        }

                        None
                    })
                    .collect::<Vec<_>>(),
            ]
            .concat()
        })
        .collect::<Vec<_>>();
    unsorted_points.dedup();

    if unsorted_points.is_empty() {
        return None;
    }

    let first = unsorted_points.remove(0);
    let mut point_groups = vec![vec![first]];
    let mut current_point = first;

    while !unsorted_points.is_empty() {
        let mut points = Vec::new();

        loop {
            let mut distance = i32::MAX;
            let mut index = 0;

            for (i, point) in unsorted_points.iter().enumerate() {
                let dist = current_point.distance(*point) as i32;
                if dist < distance {
                    index = i;
                    distance = dist;
                }
            }

            points.push(current_point);
            current_point = unsorted_points.remove(index);

            if unsorted_points.is_empty() {
                points.push(current_point);
            }

            if distance >= HEX_SIZE as i32 || unsorted_points.is_empty() {
                break;
            }
        }

        points.push(*points.first()?);
        point_groups.push(points);
    }
    Some(point_groups)
}
