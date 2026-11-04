use bevy::prelude::*;
use serde::Deserialize;

#[derive(Resource, Deserialize, Clone, Debug)]
pub struct MovementConfig {
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
}

#[derive(Resource, Deserialize, Clone, Debug)]
pub struct DashConfig {
    pub speed: f32,
    pub duration_ticks: u32,
    pub cooldown_ticks: u32,
    pub max_charges: u32,
    pub i_frame_start_tick: u32,
    pub i_frame_end_tick: u32,
}

pub fn load_movement_config() -> MovementConfig {
    let path = "assets/config/movement.ron";
    let contents =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read {path}"));
    ron::from_str(&contents).unwrap_or_else(|e| panic!("Failed to parse {path}: {e}"))
}

pub fn load_dash_config() -> DashConfig {
    let path = "assets/config/dash.ron";
    let contents =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read {path}"));
    ron::from_str(&contents).unwrap_or_else(|e| panic!("Failed to parse {path}: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_config_deserializes() {
        let ron_str = "(max_speed: 8.0, acceleration: 50.0, deceleration: 40.0)";
        let config: MovementConfig = ron::from_str(ron_str).unwrap();
        assert!((config.max_speed - 8.0).abs() < f32::EPSILON);
        assert!((config.acceleration - 50.0).abs() < f32::EPSILON);
        assert!((config.deceleration - 40.0).abs() < f32::EPSILON);
    }

    #[test]
    fn dash_config_deserializes() {
        let ron_str = "(speed: 22.0, duration_ticks: 10, cooldown_ticks: 30, max_charges: 2, i_frame_start_tick: 2, i_frame_end_tick: 8)";
        let config: DashConfig = ron::from_str(ron_str).unwrap();
        assert!((config.speed - 22.0).abs() < f32::EPSILON);
        assert_eq!(config.duration_ticks, 10);
        assert_eq!(config.max_charges, 2);
        assert_eq!(config.i_frame_start_tick, 2);
        assert_eq!(config.i_frame_end_tick, 8);
    }
}
