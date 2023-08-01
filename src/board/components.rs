use bevy::prelude::*;

use crate::hexagon::Cube;

use super::resources::HexColors;

#[derive(Component)]
pub struct Border;

#[derive(Reflect, PartialEq, Debug, Clone, Copy)]
pub enum Team {
    Ally,
    Enemy,
}

#[derive(Reflect, PartialEq, Debug)]
pub enum TileVariant {
    Neutral,
    Captured(Team),
    Capital(Team),
}

#[derive(Component, PartialEq)]
pub struct HexTile {
    pub coordinate: Cube,
    pub variant: TileVariant,
    pub capture_progress: i32,
}

impl HexTile {
    pub fn strong_highlight(&self, colors: &Res<HexColors>) -> Handle<ColorMaterial> {
        match self.variant {
            TileVariant::Capital(Team::Ally) => colors.ally_capital_strong_highlight.clone(),
            TileVariant::Capital(Team::Enemy) => colors.enemy_capital_strong_highlight.clone(),
            _ => colors.neutral_strong_highlight.clone(),
        }
    }

    pub fn weak_highlight(&self, colors: &Res<HexColors>) -> Handle<ColorMaterial> {
        match self.variant {
            TileVariant::Capital(Team::Ally) => colors.ally_capital_weak_highlight.clone(),
            TileVariant::Capital(Team::Enemy) => colors.enemy_capital_weak_highlight.clone(),
            _ => colors.neutral_weak_highlight.clone(),
        }
    }

    pub fn base_color(&self, colors: &Res<HexColors>) -> Handle<ColorMaterial> {
        match self.variant {
            TileVariant::Capital(Team::Ally) => colors.ally_capital.clone(),
            TileVariant::Capital(Team::Enemy) => colors.enemy_capital.clone(),
            _ => colors.neutral.clone(),
        }
    }
}
