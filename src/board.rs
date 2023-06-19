use std::{
    cmp::{self},
    f32::consts::PI,
};

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

#[derive(Reflect)]
struct Cube {
    q: i32,
    r: i32,
    s: i32,
}

impl Cube {
    pub fn cube_new(q: i32, r: i32, s: i32) -> Self {
        Self { q, r, s}
    }

    pub fn axial_new(q: i32, r: i32) -> Self {
        Self { q, r, s: -q -r}
    }
}

#[derive(Reflect)]
struct FractionalCube {
    q: f32,
    r: f32,
    s: f32,
}

impl FractionalCube {
    pub fn cube_new(q: f32, r: f32, s: f32) -> Self {
        Self { q, r, s}
    }

    pub fn axial_new(q: f32, r: f32) -> Self {
        Self { q, r, s: -q -r}
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HexTile>()
            .insert_resource(BoardVariables {
                hex_size: 40.,
                hex_gap: 2.5,
                radius: 5,
            })
            .add_startup_system(load_colors.in_base_set(CoreSet::First))
            .add_startup_system(build_board)
            .add_system(highlight_hovered_hex);
    }
}

#[derive(Component, Reflect)]
pub struct HexTile {
    coordinate: Cube,
    variant: TileVariant,
}
#[derive(Reflect)]
enum TileVariant {
    Default,
    Player,
    Enemy,
}

#[derive(Resource)]
struct BoardVariables {
    hex_size: f32,
    hex_gap: f32,
    radius: i32,
}

#[derive(Resource)]
struct HexColors {
    backround_hex: Handle<ColorMaterial>,

    default: Handle<ColorMaterial>,
    default_hovered: Handle<ColorMaterial>,

    player: Handle<ColorMaterial>,
    player_hovered: Handle<ColorMaterial>,

    enemy: Handle<ColorMaterial>,
    enemy_hovered: Handle<ColorMaterial>,
}

fn load_colors (
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let colors = HexColors {
        backround_hex: materials.add(ColorMaterial::from(Color::rgb_u8(30, 30, 30))),

        default: materials.add(ColorMaterial::from(Color::rgb_u8(40, 40, 40))),
        default_hovered: materials.add(ColorMaterial::from(Color::rgb_u8(50, 50, 50))),

        player: materials.add(ColorMaterial::from(Color::rgb_u8(60, 60, 255))),
        player_hovered: materials.add(ColorMaterial::from(Color::rgb_u8(100, 100, 255))),

        enemy: materials.add(ColorMaterial::from(Color::rgb_u8(255, 60, 60))),
        enemy_hovered: materials.add(ColorMaterial::from(Color::rgb_u8(255, 100, 100))),
    };

    commands.insert_resource(colors);
}

fn build_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<HexColors>,
    board_vars: Res<BoardVariables>,
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
        material: colors.default.clone(),
        transform: Transform::from_scale(Vec3::new(board_vars.hex_size, board_vars.hex_size, 0.)),
        ..default()
    };

    // https://www.redblobgames.com/grids/hexagons/implementation.html#shape-hexagon
    let radius = board_vars.radius;
    let padded_size = board_vars.hex_size + board_vars.hex_gap;

    for q in -radius..=radius {
        let r1 = cmp::max(-radius, -q - radius);
        let r2 = cmp::min(radius, -q + radius);

        for r in r1..=r2 {
            // https://www.redblobgames.com/grids/hexagons/#hex-to-pixel
            let x = 3_f32.sqrt() * q as f32 + 3_f32.sqrt() / 2. * r as f32;
            let y = 3. / 2. * r as f32;
            pointy_top_hex_mesh.transform.translation = Vec3::new(x * padded_size, y * padded_size, 1.);

            if q == 0 && r == 0 {
                commands
                .spawn(pointy_top_hex_mesh.clone())
                .insert(HexTile { coordinate: Cube::axial_new(q, r), variant: TileVariant::Player});
                continue;
            }

            if q == 2 && r == 2 {
                commands
                .spawn(pointy_top_hex_mesh.clone())
                .insert(HexTile { coordinate: Cube::axial_new(q, r), variant: TileVariant::Enemy});
                continue;
            }

            commands
                .spawn(pointy_top_hex_mesh.clone())
                .insert(HexTile { coordinate: Cube::axial_new(q, r), variant: TileVariant::Default});
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

    // magic number is the x val of verticy
    // with index 2 on a pointy hex
    let magic_number = (PI / 180. * 30.).cos();
    let scale = ((board_vars.radius as f32 * 2. + 1.8) * board_vars.hex_size + board_vars.radius as f32 * 2. * board_vars.hex_gap) * magic_number;

    let flat_top_hex_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(triangle).into(),
        material: colors.backround_hex.clone(),
        transform: Transform::from_scale(Vec3::new(scale, scale, 0.)),
        ..default()
    };

    commands
        .spawn(flat_top_hex_mesh.clone());
}

fn highlight_hovered_hex(
    windows: Query<&Window>,
    mut hexes: Query<(&HexTile, &mut Handle<ColorMaterial>)>,
    board_vars: Res<BoardVariables>,
    colors: Res<HexColors>,
) {

    let Some(hovered_hex) = cursor_to_hex(windows, board_vars) else {
        return;
    };

    for (hex, mut color_mat) in &mut hexes {
        *color_mat = match hex.variant {
            TileVariant::Default => colors.default.clone(),
            TileVariant::Player => colors.player.clone(),
            TileVariant::Enemy => colors.enemy.clone(),
        };

        if hex.coordinate.q != hovered_hex.q {
            continue;
        }

        if hex.coordinate.r != hovered_hex.r {
            continue;
        }
        
        *color_mat = match hex.variant {
            TileVariant::Default => colors.default_hovered.clone(),
            TileVariant::Player => colors.player_hovered.clone(),
            TileVariant::Enemy => colors.enemy_hovered.clone(),
        };
    }
}

fn cursor_to_hex(windows: Query<&Window>, board_vars: Res<BoardVariables>) -> Option<Cube> {
    let Ok(primary) = windows.get_single() else {
        return None;
    };

    let Some(cursor_pos) = primary.cursor_position() else {
        return None;
    };

    let padded_size = board_vars.hex_size + board_vars.hex_gap;


    let x = cursor_pos.x - primary.resolution.width() / 2.;
    let y = cursor_pos.y - primary.resolution.height() / 2.;

    // https://www.redblobgames.com/grids/hexagons/#pixel-to-hex
    let q = (3_f32.sqrt() / 3. * x - 1. / 3. * y) / padded_size;
    let r = (2. / 3. * y) / padded_size;

    return Some(cube_round(FractionalCube::axial_new(q, r)));
}

fn cube_round(frac: FractionalCube) -> Cube {
    let q = frac.q.round();
    let r = frac.r.round();
    let s = frac.s.round();

    let q_diff = (q - frac.q).abs();
    let r_diff = (r - frac.r).abs();
    let s_diff = (s - frac.s).abs();

    if q_diff > r_diff && q_diff > s_diff {
        return Cube::cube_new((-r - s) as i32, r as i32, s as i32);
    }
    else if r_diff > s_diff {
        return Cube::cube_new(q as i32, (-q - s) as i32, s as i32);
    }
    else {
        return Cube::cube_new(q as i32, r as i32, (-q - r) as i32);
    }
}