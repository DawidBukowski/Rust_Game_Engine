pub mod components;
pub mod config;
pub mod systems;

use bevy::prelude::*;

use self::config::load_pushbox_config;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(load_pushbox_config());
    }
}
