use crate::components::*;
use bevy::ecs::system::Commands;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;
use bevy::time::Timer;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rascal_shoot)
            .add_systems(Update, projectile_velocity);
    }
}

#[derive(Component)]
pub struct Projectile {
    pub timer: Timer,
}

fn rascal_shoot(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &Player, &Sprite)>,
) {
    if keyboard_input.just_pressed(KeyCode::ShiftRight) {
        for (transform, player, sprite) in &mut query {
            let mut multiplier_x = 1.0;

            if sprite.flip_x == true {
                multiplier_x = -1.0;
            }
            if player.id == 2 {
                commands.spawn((
                    Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::new(5.0, 5.0)),
                        ..default()
                    },
                    Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y + sprite.custom_size.unwrap().y / 2.0,
                        transform.translation.z,
                    ),
                    Velocity(Vec2::new(multiplier_x * 400.0, 0.0)),
                    Projectile {
                        timer: Timer::from_seconds(1.0, TimerMode::Once),
                    },
                ));
            }
        }
    }
}

fn projectile_velocity(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut Velocity,
        &mut Transform,
        &mut Projectile,
        &Sprite,
    )>,
    wall_query: Query<
        (&mut Transform, &Sprite),
        (With<Wall>, Without<Player>, Without<Projectile>),
    >,
) {
    for (entity, velocity, mut transform, mut projectile, sprite) in &mut query {
        projectile.timer.tick(time.delta());
        let mut destroyed = false;

        let pos_x = transform.translation.x + velocity.0.x * time.delta_secs();
        let pos_y = transform.translation.y + velocity.0.y * time.delta_secs();

        let projectile_halfsize = sprite.custom_size.unwrap_or(Vec2::new(5.0, 5.0)) / 2.0;
        let projectile_box = Aabb2d::new(Vec2 { x: pos_x, y: pos_y }, projectile_halfsize);

        for (transform_wall, sprite_wall) in &wall_query {
            let wall_half_size = sprite_wall.custom_size.unwrap() / 2.0;
            let wall_box = Aabb2d::new(transform_wall.translation.truncate(), wall_half_size);

            if wall_box.intersects(&projectile_box) {
                commands.entity(entity).insert(DespawnEntity);
                destroyed = true;
                break;
            }
        }

        if destroyed {
            continue;
        }
        if projectile.timer.just_finished() {
            transform.translation.x += (projectile.timer.elapsed().as_secs_f32()
                - projectile.timer.duration().as_secs_f32())
                * velocity.0.x;
            if let Ok(mut entity_cmds) = commands.get_entity(entity) {
                entity_cmds.insert(DespawnEntity);
            } else {
                dbg!("ENTITY GONE AT VELOCITY");
            }
        } else {
            transform.translation.x = pos_x;
            transform.translation.y = pos_y;
        }
    }
}
