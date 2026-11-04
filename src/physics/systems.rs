use bevy::prelude::*;

use crate::core::time::SimulationControl;
use crate::physics::components::Velocity;

pub fn apply_velocity(
    sim: Res<SimulationControl>,
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    if sim.paused {
        return;
    }

    let dt = time.delta_secs();

    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.value.x * dt;
        transform.translation.z += velocity.value.y * dt;
    }
}
