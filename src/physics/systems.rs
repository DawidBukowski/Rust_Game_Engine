use bevy::prelude::*;

use crate::core::time::SimulationControl;
use crate::input::components::Player;
use crate::physics::components::{Pillar, Pushbox, Velocity};
use crate::physics::config::PushboxConfig;

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

pub fn compute_circle_push(
    player_pos: Vec2,
    player_radius: f32,
    obstacle_pos: Vec2,
    obstacle_radius: f32,
) -> Vec2 {
    let delta = player_pos - obstacle_pos;
    let dist = delta.length();
    let min_dist = player_radius + obstacle_radius;

    if dist < min_dist && dist > f32::EPSILON {
        (delta / dist) * (min_dist - dist)
    } else {
        Vec2::ZERO
    }
}

#[allow(clippy::type_complexity)]
pub fn resolve_pushbox(
    mut player_query: Query<(&mut Transform, &Pushbox), With<Player>>,
    obstacle_query: Query<(&Transform, &Pushbox), (With<Pillar>, Without<Player>)>,
) {
    let Ok((mut player_tf, player_pb)) = player_query.single_mut() else {
        return;
    };

    for (obstacle_tf, obstacle_pb) in &obstacle_query {
        let player_xz = Vec2::new(player_tf.translation.x, player_tf.translation.z);
        let obstacle_xz = Vec2::new(obstacle_tf.translation.x, obstacle_tf.translation.z);

        let push =
            compute_circle_push(player_xz, player_pb.radius, obstacle_xz, obstacle_pb.radius);
        player_tf.translation.x += push.x;
        player_tf.translation.z += push.y;
    }
}

pub fn clamp_position_to_arena(pos: Vec2, radius: f32, arena_half_extent: f32) -> Vec2 {
    let limit = arena_half_extent - radius;
    Vec2::new(pos.x.clamp(-limit, limit), pos.y.clamp(-limit, limit))
}

pub fn clamp_to_arena(
    config: Res<PushboxConfig>,
    mut query: Query<(&mut Transform, &Pushbox), With<Player>>,
) {
    let Ok((mut transform, pushbox)) = query.single_mut() else {
        return;
    };

    let clamped = clamp_position_to_arena(
        Vec2::new(transform.translation.x, transform.translation.z),
        pushbox.radius,
        config.arena_half_extent,
    );
    transform.translation.x = clamped.x;
    transform.translation.z = clamped.y;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_push_separates_overlapping() {
        let push = compute_circle_push(Vec2::new(1.0, 0.0), 0.4, Vec2::new(0.5, 0.0), 0.5);
        assert!((push.x - 0.4).abs() < 0.01);
        assert!(push.y.abs() < 0.01);
    }

    #[test]
    fn circle_push_no_overlap_returns_zero() {
        let push = compute_circle_push(Vec2::new(5.0, 0.0), 0.4, Vec2::new(0.0, 0.0), 0.5);
        assert_eq!(push, Vec2::ZERO);
    }

    #[test]
    fn circle_push_coincident_centers_returns_zero() {
        let push = compute_circle_push(Vec2::new(1.0, 1.0), 0.4, Vec2::new(1.0, 1.0), 0.5);
        assert_eq!(push, Vec2::ZERO);
    }

    #[test]
    fn arena_clamp_limits_out_of_bounds() {
        let result = clamp_position_to_arena(Vec2::new(25.0, -25.0), 0.4, 19.5);
        assert!((result.x - 19.1).abs() < 0.001);
        assert!((result.y - (-19.1)).abs() < 0.001);
    }

    #[test]
    fn arena_clamp_preserves_inside_position() {
        let result = clamp_position_to_arena(Vec2::new(5.0, -3.0), 0.4, 19.5);
        assert!((result.x - 5.0).abs() < f32::EPSILON);
        assert!((result.y - (-3.0)).abs() < f32::EPSILON);
    }
}
