use bevy::prelude::*;

use crate::debug::DebugState;
use crate::spatial::raycast::CursorWorldPosition;

pub fn draw_debug_gizmos_system(
    mut gizmos: Gizmos,
    debug_state: Res<DebugState>,
    cursor_pos: Res<CursorWorldPosition>,
) {
    if !debug_state.show_gizmos {
        return;
    }

    draw_origin_axes(&mut gizmos);
    draw_ground_grid(&mut gizmos);

    if let Some(pos) = cursor_pos.position {
        draw_cursor_gizmo(&mut gizmos, pos);
    }
}

fn draw_origin_axes(gizmos: &mut Gizmos) {
    let length = 2.0;
    let y_offset = Vec3::new(0.0, 0.01, 0.0);

    gizmos.line(
        y_offset,
        Vec3::new(length, 0.0, 0.0) + y_offset,
        Color::srgb(1.0, 0.2, 0.2),
    );
    gizmos.line(
        y_offset,
        Vec3::new(0.0, length, 0.0),
        Color::srgb(0.2, 1.0, 0.2),
    );
    gizmos.line(
        y_offset,
        Vec3::new(0.0, 0.0, length) + y_offset,
        Color::srgb(0.2, 0.2, 1.0),
    );
}

fn draw_ground_grid(gizmos: &mut Gizmos) {
    let half_size = 20;
    let color = Color::srgba(0.4, 0.4, 0.4, 0.3);
    let y = 0.005;

    for i in -half_size..=half_size {
        let f = i as f32;
        gizmos.line(
            Vec3::new(f, y, -half_size as f32),
            Vec3::new(f, y, half_size as f32),
            color,
        );
        gizmos.line(
            Vec3::new(-half_size as f32, y, f),
            Vec3::new(half_size as f32, y, f),
            color,
        );
    }
}

fn draw_cursor_gizmo(gizmos: &mut Gizmos, pos: Vec3) {
    let y = pos.y + 0.02;
    let center = Vec3::new(pos.x, y, pos.z);
    let color = Color::srgb(1.0, 1.0, 0.2);

    gizmos.circle(
        Isometry3d::from_translation(center)
            * Isometry3d::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        0.3,
        color,
    );

    let arm = 0.5;
    gizmos.line(
        center + Vec3::new(-arm, 0.0, 0.0),
        center + Vec3::new(arm, 0.0, 0.0),
        color,
    );
    gizmos.line(
        center + Vec3::new(0.0, 0.0, -arm),
        center + Vec3::new(0.0, 0.0, arm),
        color,
    );
}
