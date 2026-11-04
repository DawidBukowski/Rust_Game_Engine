use bevy::prelude::*;

use crate::input::components::Player;

#[derive(Component)]
pub struct IsometricCamera;

#[derive(Resource, Default)]
pub struct CameraTarget {
    pub position: Vec3,
}

#[derive(Resource)]
pub struct CameraConfig {
    pub fov: f32,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub follow_smoothing: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            fov: 0.52,
            distance: 25.0,
            yaw: std::f32::consts::FRAC_PI_4,
            pitch: -0.61,
            follow_smoothing: 8.0,
        }
    }
}

pub fn compute_camera_transform(config: &CameraConfig, look_at: Vec3) -> Transform {
    let horizontal_dist = config.distance * config.pitch.cos();
    let vertical_dist = config.distance * config.pitch.sin().abs();

    let offset = Vec3::new(
        horizontal_dist * config.yaw.cos(),
        vertical_dist,
        horizontal_dist * config.yaw.sin(),
    );

    let position = look_at + offset;
    Transform::from_translation(position).looking_at(look_at, Vec3::Y)
}

pub fn spawn_camera_system(mut commands: Commands, config: Res<CameraConfig>) {
    let transform = compute_camera_transform(&config, Vec3::ZERO);

    commands.spawn((
        IsometricCamera,
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: config.fov,
            ..default()
        }),
        transform,
    ));
}

pub fn camera_follow_player(
    config: Res<CameraConfig>,
    mut target: ResMut<CameraTarget>,
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<IsometricCamera>)>,
    mut camera_query: Query<&mut Transform, With<IsometricCamera>>,
) {
    let Ok(player_tf) = player_query.single() else {
        return;
    };
    let Ok(mut camera_tf) = camera_query.single_mut() else {
        return;
    };

    let player_ground = Vec3::new(player_tf.translation.x, 0.0, player_tf.translation.z);
    let dt = time.delta_secs();
    let t = (config.follow_smoothing * dt).min(1.0);
    target.position = target.position.lerp(player_ground, t);

    *camera_tf = compute_camera_transform(&config, target.position);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_transform_looks_at_target() {
        let config = CameraConfig::default();
        let transform = compute_camera_transform(&config, Vec3::ZERO);

        assert!(
            transform.translation.y > 0.0,
            "Camera should be above ground"
        );

        let forward = transform.forward();
        assert!(forward.y < 0.0, "Camera should look downward");
    }

    #[test]
    fn camera_transform_respects_distance() {
        let config = CameraConfig {
            distance: 10.0,
            ..default()
        };
        let transform = compute_camera_transform(&config, Vec3::ZERO);
        let actual_dist = transform.translation.length();
        assert!(
            (actual_dist - 10.0).abs() < 1.0,
            "Camera distance should be approximately the configured distance"
        );
    }

    #[test]
    fn camera_target_lerp_moves_toward_player() {
        let config = CameraConfig::default();
        let mut target = CameraTarget::default();

        let player_ground = Vec3::new(5.0, 0.0, 3.0);
        let dt = 1.0 / 60.0;
        let t = (config.follow_smoothing * dt).min(1.0);
        target.position = target.position.lerp(player_ground, t);

        assert!(
            target.position.x > 0.0,
            "Target should move toward player X"
        );
        assert!(
            target.position.z > 0.0,
            "Target should move toward player Z"
        );
        assert!(
            target.position.x < 5.0,
            "Target should not overshoot player X"
        );
        assert!(
            target.position.z < 3.0,
            "Target should not overshoot player Z"
        );
    }
}
