use bevy::prelude::*;

use super::components::Unit;

#[derive(Resource)]
pub struct SelectedUnit(pub Option<Unit>);
