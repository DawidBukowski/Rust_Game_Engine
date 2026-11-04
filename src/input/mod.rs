pub mod components;
pub mod config;
pub mod messages;
pub mod systems;

use bevy::prelude::*;

use self::config::{load_dash_config, load_movement_config};
use self::messages::{ActionRequested, DashStarted};
use self::systems::{read_raw_input, resolve_movement_intent};
use crate::core::sets::GameSet;
use crate::core::time::tick_counter_system;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(load_movement_config());
        app.insert_resource(load_dash_config());

        app.add_message::<ActionRequested>();
        app.add_message::<DashStarted>();

        app.add_systems(
            FixedUpdate,
            read_raw_input
                .in_set(GameSet::Input)
                .after(tick_counter_system),
        );
        app.add_systems(FixedUpdate, resolve_movement_intent.in_set(GameSet::Intent));
    }
}
