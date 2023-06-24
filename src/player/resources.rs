use bevy::prelude::*;

use crate::hexagon::Cube;

#[derive(Resource)]
pub struct AllyCapital {
    pub position: Option<Cube>,
}