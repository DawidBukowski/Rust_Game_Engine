use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity {
    pub value: Vec2,
}

#[derive(Component)]
pub struct Pushbox {
    pub radius: f32,
}

#[derive(Component)]
pub struct Hurtbox {
    pub radius: f32,
}

#[derive(Component)]
pub struct Pillar;
