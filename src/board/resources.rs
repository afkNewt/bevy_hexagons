use bevy::prelude::*;

#[derive(Resource)]
pub struct HexColors {
    pub backround_hex: Handle<ColorMaterial>,

    pub neutral: Handle<ColorMaterial>,
    pub neutral_hovered: Handle<ColorMaterial>,

    pub ally_capital: Handle<ColorMaterial>,
    pub ally_capital_hovered: Handle<ColorMaterial>,

    pub enemy_capital: Handle<ColorMaterial>,
    pub enemy_capital_hovered: Handle<ColorMaterial>,
}