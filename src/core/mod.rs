pub mod sets;
pub mod time;

use bevy::prelude::*;

use self::sets::configure_game_sets;
use self::time::{SimulationControl, apply_time_scale_system, tick_counter_system};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(60.0));
        app.init_resource::<SimulationControl>();

        configure_game_sets(app);

        app.add_systems(
            FixedUpdate,
            tick_counter_system.in_set(sets::GameSet::Input),
        );
        app.add_systems(Update, apply_time_scale_system);
    }
}
