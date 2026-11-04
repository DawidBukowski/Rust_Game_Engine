use bevy::prelude::*;
use serde::Deserialize;

#[derive(Resource, Deserialize, Clone, Debug)]
pub struct PushboxConfig {
    pub player_radius: f32,
    pub pillar_radius: f32,
    pub arena_half_extent: f32,
}

pub fn load_pushbox_config() -> PushboxConfig {
    let path = "assets/config/pushbox.ron";
    let contents =
        std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read {path}"));
    ron::from_str(&contents).unwrap_or_else(|e| panic!("Failed to parse {path}: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pushbox_config_deserializes() {
        let ron_str = "(player_radius: 0.4, pillar_radius: 0.5, arena_half_extent: 19.5)";
        let config: PushboxConfig = ron::from_str(ron_str).unwrap();
        assert!((config.player_radius - 0.4).abs() < f32::EPSILON);
        assert!((config.pillar_radius - 0.5).abs() < f32::EPSILON);
        assert!((config.arena_half_extent - 19.5).abs() < f32::EPSILON);
    }
}
