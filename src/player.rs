use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ggrs::*;

use crate::{resources::PlayersPositionsBackup, *};

pub fn spawn(
    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
    models: Res<assets::ModelAssets>,
) {
    info!("Spawning players");

    // Player 1
    commands.spawn((
        components::Player { handle: 0 },
        components::BulletReady(true),
        components::MoveSpeed(0.0),
        components::RotationSpeed(0.0),
        rip.next(),
        SceneBundle {
            scene: models.tank_green.clone(),
            transform: Transform::from_translation(Vec3::new(-2., 0.0, 0.0)),
            ..default()
        },
    ));

    // Player 2
    commands.spawn((
        components::Player { handle: 1 },
        components::BulletReady(true),
        components::MoveSpeed(0.0),
        components::RotationSpeed(0.0),
        rip.next(),
        SceneBundle {
            scene: models.tank_red.clone(),
            transform: Transform::from_translation(Vec3::new(2., 0.0, 0.0)),
            ..default()
        },
    ));
}

pub fn moving(
    inputs: Res<PlayerInputs<networking::GgrsConfig>>,
    mut player_query: Query<(
        &mut Transform,
        &mut components::MoveSpeed,
        &mut components::RotationSpeed,
        &components::Player,
    )>,
) {
    for (mut transform, mut move_speed, mut rotation_speed, player) in player_query.iter_mut() {
        let (input, _) = inputs[player.handle];

        let rotation_speed_max = 0.07;
        let rotation_acceleration = 0.005;
        if rotation_speed.0.abs() < rotation_speed_max {
            rotation_speed.0 += rotation_acceleration * input::rotation(input);
        } else {
            rotation_speed.0 = rotation_speed
                .0
                .clamp(-rotation_speed_max, rotation_speed_max);
        }

        let movement_speed_max = 0.13;
        let movement_acceleration = 0.01;
        if move_speed.0.abs() < movement_speed_max {
            move_speed.0 += movement_acceleration * input::forward(input);
        } else {
            move_speed.0 = move_speed.0.clamp(-movement_speed_max, movement_speed_max);
        }

        rotation_speed.0 *= 0.9;
        move_speed.0 *= 0.9;

        transform.rotate_y(rotation_speed.0);

        let movement_direction = transform.rotation * Vec3::Z;
        let movement_distance = movement_direction * move_speed.0;
        transform.translation += movement_distance;

        let limit = Vec2::splat(constants::MAP_SIZE as f32 / 2. - 0.5);
        let new_pos = transform.translation.xz().clamp(-limit, limit);
        transform.translation = Vec3::new(new_pos.x, 0.0, new_pos.y);
    }
}

pub fn save_positions(
    mut positions: ResMut<PlayersPositionsBackup>,
    player_query: Query<(&Transform, &components::Player)>,
) {
    positions.0.clear();
    for (transform, player) in player_query.iter() {
        positions
            .0
            .push((player.handle, transform.translation.xz()));
    }
}

pub fn collision(
    positions: Res<PlayersPositionsBackup>,
    mut player_query: Query<(&mut Transform, &components::Player)>,
) {
    for (mut transform_a, player_a) in player_query.iter_mut() {
        for (handle_b, pos_b) in positions.0.iter().cloned() {
            if player_a.handle != handle_b {
                let pos_a = transform_a.translation.xz();
                let direction = pos_a - pos_b;
                let distance_squared = direction.length_squared();
                let double_radius_squared: f32 = (constants::PLAYER_RADIUS * 2.0).powi(2i32);
                if distance_squared <= double_radius_squared {
                    const PENETRATION_DEPTH: f32 = 10.0;
                    let penetration =
                        (double_radius_squared - distance_squared).sqrt() / PENETRATION_DEPTH;
                    let new_pos = pos_a + direction.normalize() * penetration;
                    let limit = Vec2::splat(constants::MAP_SIZE as f32 / 2. - 0.5);
                    let new_pos = new_pos.clamp(-limit, limit);
                    transform_a.translation.x = new_pos.x;
                    transform_a.translation.z = new_pos.y;
                }
            }
        }
    }
}
