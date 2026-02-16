use bevy::prelude::*;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Player {
    pub id: u8,
}

#[derive(Component)]
pub struct Name(pub String);

#[derive(Resource)]
pub struct FpsTimer(pub Timer);

