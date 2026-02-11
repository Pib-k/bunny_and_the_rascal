use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 400.0;
#[derive(Resource)]
struct FpsTimer(Timer);

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

#[derive(Component)]
struct Player {
    id: u8,
}

#[derive(Component)]
struct Name(String);

fn spawn_players(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // Player 1
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.5, 1.0), // Note: Color::srgb instead of Color::rgb
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(-300.0, 0.0, 0.0),
        Player { id: 1 },
    ));

    // Player 2
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.3, 0.3),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(100.0, 0.0, 0.0),
        Player { id: 2 },
    ));
}

fn move_players(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Access keyboard
    time: Res<Time>,                           // Access time (to keep speed consistent)
    mut query: Query<(&mut Transform, &Player, &Sprite)>, // Find our players
    query_wall: Query<(&Transform, &Sprite), (With<Wall>, Without<Player>)>,
) {
    for (mut transform, player, sprite) in &mut query {
        let mut direction = Vec2::ZERO;

        // Player 1 uses WASD
        if player.id == 1 {
            if keyboard_input.pressed(KeyCode::KeyW) {
                direction.y += 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                direction.y -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                direction.x -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                direction.x += 1.0;
            }
        }

        // Player 2 uses Arrow Keys
        if player.id == 2 {
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                direction.y += 1.0;
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                direction.y -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                direction.x -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                direction.x += 1.0;
            }
        }

        // Normalize direction so moving diagonally isn't faster
        let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();

        let pos_x = transform.translation.x + move_delta.x;
        let pos_y = transform.translation.y + move_delta.y;
        let player_size = sprite.custom_size.unwrap_or(Vec2 { x: 50.0, y: 50.0 });
        let player_half_size = player_size / 2.0;
        let player_bounding = Aabb2d::new(Vec2 { x: pos_x, y: pos_y }, player_half_size);
        let mut collided = false;
        for (transfrom_wall, sprite_wall) in query_wall {
            let wall_size = sprite_wall.custom_size.unwrap() / 2.0;
            let wall_bounding = Aabb2d::new(transfrom_wall.translation.truncate(), wall_size);

            if wall_bounding.intersects(&player_bounding) {
                collided = true;
                break;
            }
        }
        if !collided {
            transform.translation.x = pos_x;
            transform.translation.y = pos_y;
        }
    }
}

#[derive(Component)]
struct FpsText;

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
    mut timer: ResMut<FpsTimer>, // Access our timer
    mut query: Query<&mut Text, With<FpsText>>,
) {
    // Tick the timer with the time elapsed since the last frame
    timer.0.tick(time.delta());

    // Only run the update logic if the timer just finished its 3-second cycle
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

#[derive(Component)]
struct Wall;

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
