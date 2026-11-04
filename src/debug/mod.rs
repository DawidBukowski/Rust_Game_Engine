pub mod egui_overlay;
pub mod gizmos;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use self::egui_overlay::debug_egui_system;
use self::gizmos::draw_debug_gizmos_system;

#[derive(Resource)]
pub struct DebugState {
    pub show_gizmos: bool,
    pub show_egui: bool,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            show_gizmos: true,
            show_egui: false,
        }
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default());
        app.init_resource::<DebugState>();

        app.add_systems(Update, toggle_debug_systems);
        app.add_systems(Update, draw_debug_gizmos_system);
        app.add_systems(bevy_egui::EguiPrimaryContextPass, debug_egui_system);
    }
}

fn toggle_debug_systems(keyboard: Res<ButtonInput<KeyCode>>, mut debug_state: ResMut<DebugState>) {
    if keyboard.just_pressed(KeyCode::F1) {
        debug_state.show_gizmos = !debug_state.show_gizmos;
    }
    if keyboard.just_pressed(KeyCode::F2) {
        debug_state.show_egui = !debug_state.show_egui;
    }
}
