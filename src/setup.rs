use bevy::prelude::*;

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        ..default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("player.png"),
        transform: Transform::from_xyz(100., 0., 0.),
        ..default()
    })
    .insert(Direction::Up);
}

pub fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut player, mut transform) in sprite_position.iter_mut() {
        match *player {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
        }

        if transform.translation.y > 200. {
            *player = Direction::Down;
        } else if transform.translation.y < -200. {
            *player = Direction::Up;
        }

    }
}
