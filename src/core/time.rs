use bevy::prelude::*;

#[derive(Resource)]
pub struct SimulationControl {
    pub paused: bool,
    pub single_step_requested: bool,
    pub time_scale: f32,
    pub tick_count: u64,
}

impl Default for SimulationControl {
    fn default() -> Self {
        Self {
            paused: false,
            single_step_requested: false,
            time_scale: 1.0,
            tick_count: 0,
        }
    }
}

pub fn tick_counter_system(mut sim: ResMut<SimulationControl>) {
    if sim.paused && !sim.single_step_requested {
        return;
    }

    sim.tick_count += 1;

    if sim.single_step_requested {
        sim.single_step_requested = false;
        sim.paused = true;
    }
}

pub fn apply_time_scale_system(
    sim: Res<SimulationControl>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    if sim.paused && !sim.single_step_requested {
        virtual_time.set_relative_speed(0.0);
    } else {
        virtual_time.set_relative_speed(sim.time_scale);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_counter_increments() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<SimulationControl>();
        app.add_systems(Update, tick_counter_system);
        app.update();
        let tick = app.world().resource::<SimulationControl>().tick_count;
        assert!(tick >= 1);
    }

    #[test]
    fn pause_stops_ticking() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(SimulationControl {
            paused: true,
            ..default()
        });
        app.add_systems(Update, tick_counter_system);
        app.update();
        app.update();
        let tick = app.world().resource::<SimulationControl>().tick_count;
        assert_eq!(tick, 0);
    }

    #[test]
    fn single_step_advances_one_tick_then_pauses() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(SimulationControl {
            paused: true,
            single_step_requested: true,
            ..default()
        });
        app.add_systems(Update, tick_counter_system);

        app.update();
        let sim = app.world().resource::<SimulationControl>();
        assert_eq!(sim.tick_count, 1);
        assert!(sim.paused);
        assert!(!sim.single_step_requested);

        app.update();
        let sim = app.world().resource::<SimulationControl>();
        assert_eq!(sim.tick_count, 1);
    }
}
