use bevy::prelude::*;

#[derive(Component)]
pub struct IsometricCamera;

#[derive(Resource)]
pub struct CameraConfig {
    pub fov: f32,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    #[allow(dead_code)]
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
}
