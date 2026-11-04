use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSet {
    Input,
    Intent,
    MovementPhysics,
    CombatState,
    HitDetection,
    DamageAndBoon,
    Ai,
    Cleanup,
}

pub fn configure_game_sets(app: &mut App) {
    app.configure_sets(
        FixedUpdate,
        (
            GameSet::Input,
            GameSet::Intent,
            GameSet::MovementPhysics,
            GameSet::CombatState,
            GameSet::HitDetection,
            GameSet::DamageAndBoon,
            GameSet::Ai,
            GameSet::Cleanup,
        )
            .chain(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_sets_configure_without_panic() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        configure_game_sets(&mut app);
        app.update();
    }
}
