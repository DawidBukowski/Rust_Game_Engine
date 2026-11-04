use bevy::prelude::*;

use crate::core::time::SimulationControl;
use crate::input::components::{ActionState, DashState, Facing, Player};
use crate::input::config::DashConfig;
use crate::input::messages::{ActionRequested, DashStarted, RequestedAction};
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

pub fn compute_i_frame(elapsed_ticks: u32, start: u32, end: u32) -> bool {
    elapsed_ticks >= start && elapsed_ticks < end
}

#[allow(clippy::type_complexity)]
pub fn apply_dash(
    config: Res<DashConfig>,
    mut messages: MessageReader<ActionRequested>,
    mut dash_writer: MessageWriter<DashStarted>,
    mut query: Query<
        (
            Entity,
            &mut DashState,
            &mut ActionState,
            &mut Velocity,
            &Facing,
        ),
        With<Player>,
    >,
) {
    let Ok((entity, mut dash, mut action, mut velocity, _facing)) = query.single_mut() else {
        return;
    };

    for msg in messages.read() {
        match msg.action {
            RequestedAction::Dash { direction } => {
                if *action == ActionState::Dashing || dash.charges == 0 {
                    continue;
                }

                *action = ActionState::Dashing;
                dash.elapsed_ticks = 0;
                dash.direction = direction;
                dash.charges -= 1;
                dash.cooldown_remaining = config.cooldown_ticks;
                velocity.value = direction * config.speed;

                dash_writer.write(DashStarted { entity, direction });
            }
        }
    }
}

pub fn tick_dash(
    config: Res<DashConfig>,
    mut query: Query<(&mut DashState, &mut ActionState, &mut Velocity), With<Player>>,
) {
    let Ok((mut dash, mut action, mut velocity)) = query.single_mut() else {
        return;
    };

    if *action != ActionState::Dashing && dash.charges < config.max_charges {
        if dash.cooldown_remaining > 0 {
            dash.cooldown_remaining -= 1;
        }
        if dash.cooldown_remaining == 0 {
            dash.charges += 1;
            if dash.charges < config.max_charges {
                dash.cooldown_remaining = config.cooldown_ticks;
            }
        }
    }

    if *action != ActionState::Dashing {
        return;
    }

    dash.elapsed_ticks += 1;

    dash.invulnerable = compute_i_frame(
        dash.elapsed_ticks,
        config.i_frame_start_tick,
        config.i_frame_end_tick,
    );

    velocity.value = dash.direction * config.speed;

    if dash.elapsed_ticks >= config.duration_ticks {
        *action = ActionState::Idle;
        dash.invulnerable = false;
        dash.elapsed_ticks = 0;
        dash.direction = Vec2::ZERO;
        velocity.value = Vec2::ZERO;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::components::DashState;

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

    #[test]
    fn i_frame_before_window_is_false() {
        assert!(!compute_i_frame(1, 2, 8));
    }

    #[test]
    fn i_frame_inside_window_is_true() {
        assert!(compute_i_frame(2, 2, 8));
        assert!(compute_i_frame(5, 2, 8));
        assert!(compute_i_frame(7, 2, 8));
    }

    #[test]
    fn i_frame_at_end_boundary_is_false() {
        assert!(!compute_i_frame(8, 2, 8));
        assert!(!compute_i_frame(10, 2, 8));
    }

    #[test]
    fn dash_lifecycle_i_frames() {
        let mut invulnerable_ticks = Vec::new();
        for tick in 1..=10u32 {
            if compute_i_frame(tick, 2, 8) {
                invulnerable_ticks.push(tick);
            }
        }
        assert_eq!(invulnerable_ticks, vec![2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn cooldown_regens_charge() {
        let mut dash = DashState::new(2);
        dash.charges = 1;
        dash.cooldown_remaining = 2;
        let max_charges = 2u32;

        // Tick 1: cooldown 2 → 1
        if dash.cooldown_remaining > 0 {
            dash.cooldown_remaining -= 1;
        }
        assert_eq!(dash.cooldown_remaining, 1);
        assert_eq!(dash.charges, 1);

        // Tick 2: cooldown 1 → 0, regen charge to max
        if dash.cooldown_remaining > 0 {
            dash.cooldown_remaining -= 1;
        }
        if dash.cooldown_remaining == 0 && dash.charges < max_charges {
            dash.charges += 1;
        }
        assert_eq!(dash.charges, 2);
        assert_eq!(dash.cooldown_remaining, 0);
    }
}
