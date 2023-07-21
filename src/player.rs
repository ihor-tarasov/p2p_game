use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ggrs::*;

use crate::{resources::PlayersPositionsBackup, *};

pub fn spawn_players(
    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
    models: Res<assets::ModelAssets>,
) {
    info!("Spawning players");

    // Player 1
    commands.spawn((
        components::Player { handle: 0 },
        components::BulletReady(true),
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
        rip.next(),
        SceneBundle {
            scene: models.tank_red.clone(),
            transform: Transform::from_translation(Vec3::new(2., 0.0, 0.0)),
            ..default()
        },
    ));
}

pub fn move_players(
    inputs: Res<PlayerInputs<networking::GgrsConfig>>,
    mut player_query: Query<(&mut Transform, &components::Player)>,
) {
    for (mut transform, player) in player_query.iter_mut() {
        let (input, _) = inputs[player.handle];

        transform.rotate_y(0.07 * input::rotation(input));

        let movement_factor = Vec3::ONE;
        let movement_direction = transform.rotation * Vec3::Z;
        // get the distance the ship will move based on direction, the ship's movement speed and delta time
        let movement_distance = movement_factor * input::forward(input) * 0.13;
        // create the change in translation using the new movement direction and distance
        let translation_delta = movement_direction * movement_distance;
        // update the ship translation with our new translation delta
        transform.translation += translation_delta;
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

pub fn players_collision(
    positions: Res<PlayersPositionsBackup>,
    mut player_query: Query<(&mut Transform, &components::Player)>,
) {
    for (mut transform_a, player_a) in player_query.iter_mut() {
        for (handle_b, pos_b) in positions.0.iter().cloned() {
            if player_a.handle != handle_b {
                let pos_a = transform_a.translation.xz();
                let dir = pos_a - pos_b;
                let distance2 = dir.length_squared();
                if distance2 <= (0.5f32 * 2.0).powi(2i32) {
                    let half_penetration = ((0.5f32 * 2.0).powi(2i32) - distance2).sqrt() / 2.0;
                    let new_pos = pos_a + dir.normalize() * half_penetration;
                    let limit = Vec2::splat(constants::MAP_SIZE as f32 / 2. - 0.5);
                    let new_pos = new_pos.clamp(-limit, limit);
                    transform_a.translation.x = new_pos.x;
                    transform_a.translation.z = new_pos.y;
                }
            }
        }
    }
}
