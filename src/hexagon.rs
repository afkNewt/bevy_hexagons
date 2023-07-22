use std::{cmp, fmt};

use bevy::prelude::*;

use crate::board::{HEX_GAP, HEX_RADIUS, HEX_SIZE};

#[derive(Reflect, PartialEq, Clone, Copy)]
pub struct Cube {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl Cube {
    pub fn cube_new(q: i32, r: i32, s: i32) -> Self {
        Self { q, r, s }
    }

    pub fn axial_new(q: i32, r: i32) -> Self {
        Self { q, r, s: -q - r }
    }

    // 0 is to the right for pointy top hexes
    // 0 is to the bottom right for flat top hexes
    // indexes move counter-clockwise
    pub const CUBE_DIRECTION_VECTORS: [Cube; 6] = [
        Cube { q: 1, r: 0, s: -1 },
        Cube { q: 1, r: -1, s: 0 },
        Cube { q: 0, r: -1, s: 1 },
        Cube { q: -1, r: 0, s: 1 },
        Cube { q: -1, r: 1, s: 0 },
        Cube { q: 0, r: 1, s: -1 },
    ];

    pub const CUBE_DIAGONAL_VECTORS: [Cube; 6] = [
        Cube { q: 2, r: -1, s: -1 },
        Cube { q: 1, r: -2, s: 1 },
        Cube { q: -1, r: -1, s: 2 },
        Cube { q: -2, r: 1, s: 1 },
        Cube { q: -1, r: 2, s: -1 },
        Cube { q: 1, r: 1, s: -2 },
    ];

    pub fn cube_direction(direction: usize) -> Cube {
        Cube::CUBE_DIRECTION_VECTORS[direction]
    }

    pub fn cube_add(self, vec: Cube) -> Cube {
        Cube::cube_new(self.q + vec.q, self.r + vec.r, self.s + vec.s)
    }

    pub fn cub_subtract(self, hex: Cube) -> Cube {
        Cube::cube_new(self.q - hex.q, self.r - hex.r, self.s - hex.s)
    }

    pub fn cube_neighbor(self, direction: usize) -> Cube {
        self.cube_add(Self::cube_direction(direction))
    }

    pub fn cube_neighbors(self) -> [Cube; 6] {
        let mut neighbors = [Cube::cube_new(0, 0, 0); 6];

        for i in 0..6 {
            neighbors[i] = Self::cube_neighbor(self, i);
        }

        return neighbors;
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.q, self.r, self.s)
    }
}

#[derive(Reflect)]
pub struct FractionalCube {
    pub q: f32,
    pub r: f32,
    pub s: f32,
}

impl FractionalCube {
    pub fn cube_new(q: f32, r: f32, s: f32) -> Self {
        Self { q, r, s }
    }

    pub fn axial_new(q: f32, r: f32) -> Self {
        Self { q, r, s: -q - r }
    }

    pub fn round(&self) -> Cube {
        let q = self.q.round();
        let r = self.r.round();
        let s = self.s.round();

        let q_diff = (q - self.q).abs();
        let r_diff = (r - self.r).abs();
        let s_diff = (s - self.s).abs();

        if q_diff > r_diff && q_diff > s_diff {
            return Cube::cube_new((-r - s) as i32, r as i32, s as i32);
        } else if r_diff > s_diff {
            return Cube::cube_new(q as i32, (-q - s) as i32, s as i32);
        } else {
            return Cube::cube_new(q as i32, r as i32, (-q - r) as i32);
        }
    }
}

pub fn cursor_to_hex(windows: Query<&Window>) -> Option<Cube> {
    let Ok(primary) = windows.get_single() else {
        return None;
    };

    let Some(cursor_pos) = primary.cursor_position() else {
        return None;
    };

    let x = cursor_pos.x - primary.resolution.width() / 2.;
    let y = (cursor_pos.y - primary.resolution.height() / 2.) * -1.;

    let mouse_pos = pixel_to_hex(x, y);

    if !hexes_in_range(HEX_RADIUS, Cube::axial_new(0, 0)).contains(&mouse_pos) {
        return None;
    }

    return Some(pixel_to_hex(x, y));
}

// https://www.redblobgames.com/grids/hexagons/#hex-to-pixel
pub fn hex_to_pixel(hex: Cube) -> (f32, f32) {
    let padded_size = HEX_SIZE + HEX_GAP;
    let x = padded_size * (3_f32.sqrt() * hex.q as f32 + 3_f32.sqrt() / 2. * hex.r as f32);
    let y = padded_size * (3. / 2. * hex.r as f32);
    return (x, y);
}

// https://www.redblobgames.com/grids/hexagons/#pixel-to-hex
pub fn pixel_to_hex(x: f32, y: f32) -> Cube {
    let padded_size = HEX_SIZE + HEX_GAP;
    let q = (3_f32.sqrt() / 3. * x - 1. / 3. * y) / padded_size;
    let r = (2. / 3. * y) / padded_size;

    return FractionalCube::axial_new(q, r).round();
}

pub fn hexes_in_range(radius: i32, center: Cube) -> Vec<Cube> {
    (-radius..=radius)
        .flat_map(|q| -> Vec<Cube> {
            let r1 = cmp::max(-radius, -q - radius);
            let r2 = cmp::min(radius, -q + radius);

            (r1..=r2)
                .map(|r| -> Cube { center.cube_add(Cube::axial_new(q, r)) })
                .collect()
        })
        .collect()
}

pub fn cube_distance(hex1: Cube, hex2: Cube) -> i32 {
    let vec = hex1.cub_subtract(hex2);
    return vec.q.abs().max(vec.r.abs()).max(vec.s.abs());
}

pub fn cube_scale_vec(hexes: Vec<Cube>, factor: i32) -> Vec<Cube> {
    hexes
        .iter()
        .map(|c| Cube::cube_new(c.q * factor, c.r * factor, c.s * factor))
        .collect()
}

pub fn cube_scale(hex: Cube, factor: i32) -> Cube {
    Cube::cube_new(hex.q * factor, hex.r * factor, hex.s * factor)
}

// https://www.redblobgames.com/grids/hexagons/#rings
pub fn hexes_in_ring(radius: i32, center: Cube) -> Vec<Cube> {
    let mut hex = center.cube_add(cube_scale(Cube::CUBE_DIRECTION_VECTORS[4], radius));

    let mut results = Vec::new();
    for i in 0..6 {
        for _ in 0..radius {
            results.push(hex);
            hex = hex.cube_neighbor(i);
        }
    }

    return results;
}
