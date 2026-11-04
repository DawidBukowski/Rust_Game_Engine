use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

mod core;
mod debug;
#[allow(dead_code)]
mod input;
#[allow(dead_code)]
mod physics;
mod spatial;

use crate::core::CorePlugin;
use crate::debug::DebugPlugin;
use crate::input::InputPlugin;
use crate::input::components::{ActionState, DashState, Facing, InputBuffer, Player};
use crate::input::config::DashConfig;
use crate::physics::PhysicsPlugin;
use crate::physics::components::{Pillar, Pushbox, Velocity};
use crate::physics::config::PushboxConfig;
use crate::spatial::SpatialPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Isometric Engine".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(CorePlugin)
        .add_plugins(InputPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(SpatialPlugin)
        .add_plugins(DebugPlugin)
        .add_systems(Startup, (spawn_arena, spawn_player))
        .add_systems(Update, handle_window_close)
        .run();
}

fn handle_window_close(
    mut close_events: MessageReader<bevy::window::WindowCloseRequested>,
    mut exit: MessageWriter<AppExit>,
) {
    for _event in close_events.read() {
        exit.write(AppExit::Success);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pushbox_config: Res<PushboxConfig>,
    dash_config: Res<DashConfig>,
) {
    commands.spawn((
        Player,
        Facing::default(),
        ActionState::default(),
        DashState::new(dash_config.max_charges),
        InputBuffer::default(),
        Velocity::default(),
        Pushbox {
            radius: pushbox_config.player_radius,
        },
        Mesh3d(meshes.add(Capsule3d::new(0.3, 0.4))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.7, 0.3),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.7, 0.0),
    ));
}

fn spawn_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pushbox_config: Res<PushboxConfig>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(40.0, 0.1, 40.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.35, 0.4),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, -0.05, 0.0),
    ));

    let pillar_positions = [
        Vec3::new(5.0, 1.0, 5.0),
        Vec3::new(-5.0, 1.0, -5.0),
        Vec3::new(8.0, 1.0, -3.0),
        Vec3::new(-7.0, 1.0, 6.0),
        Vec3::new(0.0, 1.0, -8.0),
        Vec3::new(-3.0, 1.0, 3.0),
    ];

    let pillar_colors = [
        Color::srgb(0.6, 0.3, 0.3),
        Color::srgb(0.3, 0.6, 0.3),
        Color::srgb(0.3, 0.3, 0.6),
        Color::srgb(0.6, 0.6, 0.3),
        Color::srgb(0.5, 0.3, 0.5),
        Color::srgb(0.3, 0.5, 0.5),
    ];

    let pillar_mesh = meshes.add(Cylinder::new(0.5, 2.0));

    for (pos, color) in pillar_positions.iter().zip(pillar_colors.iter()) {
        commands.spawn((
            Pillar,
            Pushbox {
                radius: pushbox_config.pillar_radius,
            },
            Mesh3d(pillar_mesh.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: *color,
                perceptual_roughness: 0.7,
                ..default()
            })),
            Transform::from_translation(*pos),
        ));
    }

    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));
}
