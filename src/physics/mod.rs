pub mod components;
pub mod config;
pub mod systems;

use bevy::prelude::*;

use self::config::load_pushbox_config;
use self::systems::apply_velocity;
use crate::core::sets::GameSet;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(load_pushbox_config());

        app.add_systems(FixedUpdate, apply_velocity.in_set(GameSet::MovementPhysics));
    }
}
