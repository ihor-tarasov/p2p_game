use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ggrs::*;

use crate::*;

pub fn reload_bullet(
    inputs: Res<PlayerInputs<networking::GgrsConfig>>,
    mut query: Query<(&mut components::BulletReady, &components::Player)>,
) {
    for (mut can_fire, player) in query.iter_mut() {
        let (input, _) = inputs[player.handle];
        if !input::fire(input) {
            can_fire.0 = true;
        }
    }
}

pub fn fire_bullets(
    mut commands: Commands,
    inputs: Res<PlayerInputs<networking::GgrsConfig>>,
    //models: Res<assets::ModelAssets>,
    mut player_query: Query<(
        &Transform,
        &components::Player,
        &mut components::BulletReady,
    )>,
    mut rip: ResMut<RollbackIdProvider>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (transform, player, mut bullet_ready) in player_query.iter_mut() {
        let (input, _) = inputs[player.handle];
        if input::fire(input) && bullet_ready.0 {
            let player_pos = transform.translation.xz();
            commands.spawn((
                components::Bullet,
                rip.next(),
                PbrBundle {
                    transform: Transform::from_translation(Vec3::new(
                        player_pos.x,
                        0.5,
                        player_pos.y,
                    ))
                    .with_rotation(transform.rotation),
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0., 0.4, 0.).into()),
                    ..default()
                },
            ));
            bullet_ready.0 = false;
        }
    }
}

pub fn move_bullet(mut query: Query<&mut Transform, With<components::Bullet>>) {
    for mut transform in query.iter_mut() {
        let movement_factor = Vec3::ONE;
        let movement_direction = transform.rotation * Vec3::Z;
        // get the distance the ship will move based on direction, the ship's movement speed and delta time
        let movement_distance = movement_factor * 1.0;
        // create the change in translation using the new movement direction and distance
        let translation_delta = movement_direction * movement_distance;
        // update the ship translation with our new translation delta
        transform.translation += translation_delta;
    }
}

const PLAYER_RADIUS: f32 = 0.5;
const BULLET_RADIUS: f32 = 0.025;

pub fn kill_players(
    mut commands: Commands,
    player_query: Query<
        (Entity, &Transform),
        (With<components::Player>, Without<components::Bullet>),
    >,
    bullet_query: Query<&Transform, With<components::Bullet>>,
) {
    for (player, player_transform) in player_query.iter() {
        for bullet_transform in bullet_query.iter() {
            let distance = Vec2::distance(
                player_transform.translation.xz(),
                bullet_transform.translation.xz(),
            );
            if distance < PLAYER_RADIUS + BULLET_RADIUS {
                commands.entity(player).despawn_recursive();
            }
        }
    }
}
