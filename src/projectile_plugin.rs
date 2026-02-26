use bevy::app::*;
use bevy::ecs::system::Commands;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::time::Timer;
use crate::components::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct Projectile {
    pub timer: Timer,
}

fn rascal_shoot(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>
    query: Query<(&Transform, &Player, &Sprite)>
    )
{
    
}
