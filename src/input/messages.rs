use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum RequestedAction {
    Dash { direction: Vec2 },
}

#[derive(Message, Debug, Clone)]
pub struct ActionRequested {
    pub entity: Entity,
    pub action: RequestedAction,
}

#[derive(Message, Debug, Clone)]
pub struct DashStarted {
    pub entity: Entity,
    pub direction: Vec2,
}
