use bevy::prelude::*;

use crate::hexagon::Cube;

#[derive(Reflect, PartialEq, Debug)]
pub enum TileVariant {
    Neutral,
    AllyLand,
    EnemyLand,
    AllyCapital,
    EnemyCapital,
}

#[derive(Component)]
pub struct HexTile {
    pub coordinate: Cube,
    pub variant: TileVariant,
    pub capture_progress: i32,
}
