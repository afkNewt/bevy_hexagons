use bevy::prelude::*;

#[derive(Resource)]
pub struct HexColors {
    pub backround_hex: Handle<ColorMaterial>,

    pub neutral: Handle<ColorMaterial>,
    pub neutral_weak_highlight: Handle<ColorMaterial>,
    pub neutral_strong_highlight: Handle<ColorMaterial>,

    pub ally_capital: Handle<ColorMaterial>,
    pub ally_capital_weak_highlight: Handle<ColorMaterial>,
    pub ally_capital_strong_highlight: Handle<ColorMaterial>,

    pub enemy_capital: Handle<ColorMaterial>,
    pub enemy_capital_weak_highlight: Handle<ColorMaterial>,
    pub enemy_capital_strong_highlight: Handle<ColorMaterial>,
}
