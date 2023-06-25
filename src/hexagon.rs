use std::fmt;

use bevy::prelude::*;

use crate::board::{HEX_SIZE, HEX_GAP};

#[derive(Reflect, PartialEq, Clone, Copy)]
pub struct Cube {
    pub q: i32,
    pub r: i32,
    pub s: i32,
}

impl Cube {
    pub fn cube_new(q: i32, r: i32, s: i32) -> Self {
        Self { q, r, s}
    }

    pub fn axial_new(q: i32, r: i32) -> Self {
        Self { q, r, s: -q -r}
    }

    // 0 is to the right for pointy top hexes
    // 0 is to the bottom right for flat top hexes
    // indexes move counter-clockwise
    const CUBE_DIRECTION_VECTORS: [Cube; 6] = [
        Cube { q: 1, r: 0, s: -1 }, Cube { q: 1, r: -1, s: 0 }, Cube { q: 0, r: -1, s: 1 },
        Cube { q: -1, r: 0, s: 1 }, Cube { q: -1, r: 1, s: 0 }, Cube { q: 0, r: 1, s: -1 },
    ];

    pub fn cube_direction(direction: usize) -> Cube {
        return Cube::CUBE_DIRECTION_VECTORS[direction];
    }

    pub fn cube_add(self, vec: Cube) -> Cube {
        return Cube::cube_new(self.q + vec.q, self.r + vec.r, self.s + vec.s);
    }

    pub fn cube_neighbor(self, direction: usize) -> Cube {
        return self.cube_add(Self::cube_direction(direction));
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
        Self { q, r, s}
    }

    pub fn axial_new(q: f32, r: f32) -> Self {
        Self { q, r, s: -q -r}
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
        }
        else if r_diff > s_diff {
            return Cube::cube_new(q as i32, (-q - s) as i32, s as i32);
        }
        else {
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

    let padded_size = HEX_SIZE + HEX_GAP;


    let x = cursor_pos.x - primary.resolution.width() / 2.;
    let y = cursor_pos.y - primary.resolution.height() / 2.;

    // https://www.redblobgames.com/grids/hexagons/#pixel-to-hex
    let q = (3_f32.sqrt() / 3. * x - 1. / 3. * y) / padded_size;
    let r = (2. / 3. * y) / padded_size;

    return Some(FractionalCube::axial_new(q, r).round());
}