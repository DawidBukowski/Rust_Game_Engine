use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

use crate::core::time::SimulationControl;
use crate::debug::DebugState;
use crate::spatial::raycast::CursorWorldPosition;

#[derive(Default)]
pub struct FpsDisplay {
    cached: f64,
    cooldown: f32,
}

const FPS_UPDATE_INTERVAL: f32 = 0.5;

pub fn debug_egui_system(
    mut contexts: EguiContexts,
    debug_state: Res<DebugState>,
    mut sim: ResMut<SimulationControl>,
    cursor_pos: Res<CursorWorldPosition>,
    diagnostics: Res<DiagnosticsStore>,
    time: Res<Time>,
    mut fps_display: Local<FpsDisplay>,
) -> Result {
    if !debug_state.show_egui {
        return Ok(());
    }

    fps_display.cooldown -= time.delta_secs();
    if fps_display.cooldown <= 0.0 {
        fps_display.cooldown = FPS_UPDATE_INTERVAL;
        fps_display.cached = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|d| d.smoothed())
            .unwrap_or(0.0);
    }

    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Debug Panel")
        .default_open(true)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Performance");
            ui.label(format!("FPS: {:.0}", fps_display.cached));

            ui.separator();
            ui.heading("Simulation");
            ui.label(format!("Tick: {}", sim.tick_count));

            ui.checkbox(&mut sim.paused, "Paused");

            if sim.paused && ui.button("Single Step (1 tick)").clicked() {
                sim.single_step_requested = true;
            }

            ui.add(egui::Slider::new(&mut sim.time_scale, 0.1..=2.0).text("Time Scale"));

            ui.separator();
            ui.heading("Cursor");
            match cursor_pos.position {
                Some(pos) => {
                    ui.label(format!("X: {:.2}  Y: {:.2}  Z: {:.2}", pos.x, pos.y, pos.z));
                }
                None => {
                    ui.label("Cursor not on ground plane");
                }
            }
        });

    Ok(())
}
