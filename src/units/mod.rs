use bevy::prelude::*;

use self::{
    resources::SelectedUnit,
    systems::{
        check_for_unit_movement, check_for_unit_selection, highlight_unit_hex, test_spawn_unit,
    },
};

pub mod components;
mod resources;
mod systems;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedUnit(None))
            .add_startup_system(test_spawn_unit.in_base_set(StartupSet::PostStartup))
            .add_systems((
                check_for_unit_selection,
                highlight_unit_hex,
                check_for_unit_movement.before(check_for_unit_selection),
            ));
    }
}
