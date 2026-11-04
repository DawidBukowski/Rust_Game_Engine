pub mod camera;
pub mod raycast;

use bevy::prelude::*;

use self::camera::{CameraConfig, CameraTarget, camera_follow_player, spawn_camera_system};
use self::raycast::{CursorWorldPosition, update_cursor_world_position};

pub struct SpatialPlugin;

impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraConfig>();
        app.init_resource::<CameraTarget>();
        app.init_resource::<CursorWorldPosition>();

        app.add_systems(Startup, spawn_camera_system);
        app.add_systems(Update, (camera_follow_player, update_cursor_world_position));
    }
}
