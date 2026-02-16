use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

mod components;
use crate::components::*;

const PLAYER_SPEED: f32 = 400.0;
fn main() {
    let background = Color::srgb(0.7, 0.2, 0.1);
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(background))
        .insert_resource(FpsTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
        .add_systems(Startup, spawn_players)
        .add_systems(Startup, setup_fps_text)
        .add_systems(Startup, spawn_walls)
        .add_systems(Update, update_fps_text)
        .add_systems(Update, move_players)
        .run();
}


fn spawn_players(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.5, 1.0), 
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(-300.0, 0.0, 0.0),
        Player { id: 1 },
    ));

    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.3, 0.3),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(300.0, 0.0, 0.0),
        Player { id: 2 },
    ));
}

fn move_players(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    time: Res<Time>,                                  
    mut query: Query<(&mut Transform, &Player, &Sprite)>, 
    query_wall: Query<(&Transform, &Sprite), (With<Wall>, Without<Player>)>,
) {
    for (mut transform, player, sprite) in &mut query {
        let mut direction = Vec2::ZERO;

        if player.id == 1 {
            if keyboard_input.pressed(KeyCode::KeyW) { direction.y += 1.0; }
            if keyboard_input.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
            if keyboard_input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
            if keyboard_input.pressed(KeyCode::KeyD) { direction.x += 1.0; }
        }

        if player.id == 2 {
            if keyboard_input.pressed(KeyCode::ArrowUp) { direction.y += 1.0; }
            if keyboard_input.pressed(KeyCode::ArrowDown) { direction.y -= 1.0; }
            if keyboard_input.pressed(KeyCode::ArrowLeft) { direction.x -= 1.0; }
            if keyboard_input.pressed(KeyCode::ArrowRight) { direction.x += 1.0; }
        }

        let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();

        let mut pos_x = transform.translation.x + move_delta.x;
        let mut pos_y = transform.translation.y + move_delta.y;

        let player_size = sprite.custom_size.unwrap_or(Vec2 { x: 50.0, y: 50.0 });
        let player_half_size = player_size / 2.0;
        let epsilon = 0.05;

        let mut player_half_size_x = player_half_size;
        player_half_size_x.y -= epsilon;

        // --- X AXIS ---
        let player_bounding_x = Aabb2d::new(
            Vec2 {
                x: pos_x,
                y: transform.translation.y,
            },
            player_half_size_x,
        );

        for (transform_wall, sprite_wall) in &query_wall {
            let wall_size = sprite_wall.custom_size.unwrap() / 2.0;
            let wall_bounding = Aabb2d::new(transform_wall.translation.truncate(), wall_size);

            if wall_bounding.intersects(&player_bounding_x) {
                if move_delta.x > 0.0 {
                    pos_x = transform_wall.translation.x - wall_size.x - player_half_size.x;
                }
                else if move_delta.x < 0.0 {
                    pos_x = transform_wall.translation.x + wall_size.x + player_half_size.x;
                }
            }
        }
        
        let mut player_half_size_y = player_half_size;
        player_half_size_y.x -= epsilon;

        // --- Y AXIS ---
        let player_bounding_y = Aabb2d::new(
            Vec2 {
                x: pos_x,
                y: pos_y,
            },
            player_half_size_y,
        );

        for (transform_wall, sprite_wall) in &query_wall {
            let wall_size = sprite_wall.custom_size.unwrap() / 2.0;
            let wall_bounding = Aabb2d::new(transform_wall.translation.truncate(), wall_size);

            if wall_bounding.intersects(&player_bounding_y) {
                if move_delta.y > 0.0 {
                    pos_y = transform_wall.translation.y - wall_size.y - player_half_size.y;
                }
                else if move_delta.y < 0.0 {
                    pos_y = transform_wall.translation.y + wall_size.y + player_half_size.y;
                }
            }
        }
        
        transform.translation.x = pos_x;
        transform.translation.y = pos_y;
    }
}

fn setup_fps_text(mut commands: Commands) {
    commands.spawn((
        Text::new("FPS: "),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        FpsText,
    ));
}

fn update_fps_text(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut timer: ResMut<FpsTimer>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        for mut text in &mut query {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    **text = format!("FPS: {:.1}", value);
                }
            }
        }
    }
}

fn spawn_walls(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(200.0, 30.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 200.0),
        Wall,
    ));
}
