use bevy::prelude::*;

use self::{
    resources::SelectedUnit,
    systems::{
        check_for_unit_movement, check_for_unit_selection, despawn_dead_units, highlight_unit_hex,
        test_spawn_unit,
    },
};

pub mod components;
mod resources;
mod systems;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedUnit(None))
            .add_systems(PostStartup, test_spawn_unit)
            .add_systems(Update, (
                check_for_unit_movement.before(check_for_unit_selection),
                check_for_unit_selection,
                despawn_dead_units,
                highlight_unit_hex,
            ));
    }
}
