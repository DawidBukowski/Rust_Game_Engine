use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
pub struct CursorWorldPosition {
    pub position: Option<Vec3>,
}

pub fn update_cursor_world_position(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut cursor_pos: ResMut<CursorWorldPosition>,
) {
    let Ok(window) = window_query.single() else {
        return;
    };
    let Some(cursor_screen) = window.cursor_position() else {
        cursor_pos.position = None;
        return;
    };
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_screen) else {
        cursor_pos.position = None;
        return;
    };

    cursor_pos.position = ray_ground_intersection(ray);
}

fn ray_ground_intersection(ray: Ray3d) -> Option<Vec3> {
    let direction_y = ray.direction.y;
    if direction_y.abs() < 1e-6 {
        return None;
    }

    let t = -ray.origin.y / direction_y;
    if t < 0.0 {
        return None;
    }

    Some(ray.origin + *ray.direction * t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_hitting_ground_returns_intersection() {
        let ray = Ray3d::new(Vec3::new(0.0, 10.0, 0.0), Dir3::NEG_Y);
        let result = ray_ground_intersection(ray);
        assert!(result.is_some());
        let point = result.unwrap();
        assert!(
            (point.y).abs() < 1e-4,
            "Intersection should be on ground plane"
        );
    }

    #[test]
    fn ray_parallel_to_ground_returns_none() {
        let ray = Ray3d::new(Vec3::new(0.0, 5.0, 0.0), Dir3::X);
        assert!(ray_ground_intersection(ray).is_none());
    }

    #[test]
    fn ray_pointing_away_returns_none() {
        let ray = Ray3d::new(Vec3::new(0.0, 10.0, 0.0), Dir3::Y);
        assert!(ray_ground_intersection(ray).is_none());
    }

    #[test]
    fn ray_at_angle_hits_correct_point() {
        let ray = Ray3d::new(
            Vec3::new(5.0, 10.0, 5.0),
            Dir3::new(Vec3::new(-1.0, -2.0, -1.0).normalize()).unwrap(),
        );
        let result = ray_ground_intersection(ray);
        assert!(result.is_some());
        let point = result.unwrap();
        assert!((point.y).abs() < 1e-3);
    }
}
