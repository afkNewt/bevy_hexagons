use bevy::prelude::*;

use crate::hexagon::Cube;

#[derive(Resource)]
pub struct AllyCapital {
    pub position: Option<Cube>,
}

#[derive(Resource)]
pub struct TurnCounter(pub i32);

#[derive(Resource)]
pub struct PlayerCoins(pub i32);
