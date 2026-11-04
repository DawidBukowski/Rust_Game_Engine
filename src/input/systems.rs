use bevy::prelude::*;

use crate::core::time::SimulationControl;
use crate::input::components::{ActionState, Facing, InputBuffer, InputSnapshot, Player};
use crate::input::config::MovementConfig;
use crate::physics::components::Velocity;
use crate::spatial::camera::CameraConfig;

pub fn read_raw_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    sim: Res<SimulationControl>,
    mut query: Query<&mut InputBuffer, With<Player>>,
) {
    let Ok(mut buffer) = query.single_mut() else {
        return;
    };

    let mut raw = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        raw.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        raw.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        raw.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        raw.x += 1.0;
    }

    let move_direction = if raw != Vec2::ZERO {
        raw.normalize()
    } else {
        Vec2::ZERO
    };

    buffer.push(InputSnapshot {
        move_direction,
        dash_requested: false,
        tick: sim.tick_count,
    });
}

pub fn camera_relative_direction(raw: Vec2, yaw: f32) -> Vec2 {
    let cam_forward = Vec2::new(-yaw.cos(), -yaw.sin());
    let cam_right = Vec2::new(yaw.sin(), -yaw.cos());
    (cam_forward * raw.y + cam_right * raw.x).normalize_or_zero()
}

pub fn resolve_movement_intent(
    camera_config: Res<CameraConfig>,
    movement_config: Res<MovementConfig>,
    time: Res<Time>,
    mut query: Query<(&InputBuffer, &mut Velocity, &mut Facing, &mut ActionState), With<Player>>,
) {
    let Ok((buffer, mut velocity, mut facing, mut state)) = query.single_mut() else {
        return;
    };

    if *state == ActionState::Dashing {
        return;
    }

    let Some(snapshot) = buffer.latest() else {
        return;
    };

    let raw = snapshot.move_direction;
    let dt = time.delta_secs();

    if raw.length_squared() > 0.001 {
        let world_dir = camera_relative_direction(raw, camera_config.yaw);

        let target_vel = world_dir * movement_config.max_speed;
        let diff = target_vel - velocity.value;
        let max_change = movement_config.acceleration * dt;

        if diff.length_squared() <= max_change * max_change {
            velocity.value = target_vel;
        } else {
            velocity.value += diff.normalize_or_zero() * max_change;
        }

        facing.direction = world_dir;
        *state = ActionState::Moving;
    } else {
        let speed = velocity.value.length();
        if speed > 0.001 {
            let decel = movement_config.deceleration * dt;
            let new_speed = (speed - decel).max(0.0);
            velocity.value = velocity.value.normalize_or_zero() * new_speed;
        } else {
            velocity.value = Vec2::ZERO;
        }

        if velocity.value.length_squared() < 0.0001 {
            *state = ActionState::Idle;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_PI_4;

    #[test]
    fn camera_relative_forward_points_into_screen() {
        let dir = camera_relative_direction(Vec2::new(0.0, 1.0), FRAC_PI_4);
        assert!(dir.x < -0.5, "W should move toward -X");
        assert!(dir.y < -0.5, "W should move toward -Z");
    }

    #[test]
    fn camera_relative_right_is_perpendicular() {
        let dir = camera_relative_direction(Vec2::new(1.0, 0.0), FRAC_PI_4);
        assert!(dir.x > 0.5, "D should move toward +X");
        assert!(dir.y < -0.5, "D should move toward -Z");
    }

    #[test]
    fn camera_relative_zero_returns_zero() {
        let dir = camera_relative_direction(Vec2::ZERO, FRAC_PI_4);
        assert_eq!(dir, Vec2::ZERO);
    }
}
