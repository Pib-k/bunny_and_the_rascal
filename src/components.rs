use bevy::prelude::*;

#[derive(Component)]
pub struct FpsText;

#[derive(Resource)]
pub struct FpsTimer(pub Timer);

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Player {
    pub id: u8,
}

#[derive(Component)]
pub struct Velocity(pub Vec2); // Velocity of entities

#[derive(Component)]
pub struct GravityScale(pub f32); // Gravity scale for entities

#[derive(Component)]
pub struct Grounded(pub bool); // Check whether an entity is grounded

#[derive(Component)]
pub struct DoubleJumping(pub bool); // Check whether an entity is double jumping

#[derive(Component)]
pub struct Hitbox(pub Vec2); // Hitbox of entities

#[derive(Component, PartialEq)]
pub enum MoveState {
    IDLE,
    RUNNING,
    JUMPING,
    FALLING,
    DASHING,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct DespawnEntity;
