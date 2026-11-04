pub mod components;
pub mod config;
pub mod messages;
pub mod systems;

use bevy::prelude::*;

use self::config::{load_dash_config, load_movement_config};
use self::messages::{ActionRequested, DashStarted};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(load_movement_config());
        app.insert_resource(load_dash_config());

        app.add_message::<ActionRequested>();
        app.add_message::<DashStarted>();
    }
}
