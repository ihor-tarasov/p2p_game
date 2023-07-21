use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ggrs::*;

use crate::*;

pub fn reload(
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

pub fn fire(
    mut commands: Commands,
    inputs: Res<PlayerInputs<networking::GgrsConfig>>,
    models: Res<assets::ModelAssets>,
    mut player_query: Query<(
        &Transform,
        &components::Player,
        &mut components::BulletReady,
    )>,
    mut rip: ResMut<RollbackIdProvider>,
) {
    for (transform, player, mut bullet_ready) in player_query.iter_mut() {
        let (input, _) = inputs[player.handle];
        if input::fire(input) && bullet_ready.0 {
            let player_pos = transform.translation.xz();
            commands.spawn((
                components::Bullet,
                rip.next(),
                SceneBundle {
                    transform: Transform::from_translation(Vec3::new(
                        player_pos.x,
                        constants::GUN_HEIGHT,
                        player_pos.y,
                    ))
                    .with_rotation(transform.rotation),
                    scene: models.bullet.clone(),
                    ..default()
                },
            ));
            bullet_ready.0 = false;
        }
    }
}

pub fn moving(mut query: Query<&mut Transform, With<components::Bullet>>) {
    for mut transform in query.iter_mut() {
        let translation_delta = transform.rotation * Vec3::Z;
        transform.translation += translation_delta;
    }
}

pub fn kill(
    mut commands: Commands,
    player_query: Query<
        (Entity, &Transform),
        (With<components::Player>, Without<components::Bullet>),
    >,
    bullet_query: Query<(Entity, &Transform), With<components::Bullet>>,
) {
    for (player, player_transform) in player_query.iter() {
        for (bullet, bullet_transform) in bullet_query.iter() {
            let distance = Vec2::distance(
                player_transform.translation.xz(),
                bullet_transform.translation.xz(),
            );
            if distance < constants::PLAYER_RADIUS + constants::BULLET_RADIUS {
                commands.entity(player).despawn_recursive();
                commands.entity(bullet).despawn_recursive();
            }
        }
    }
}

pub fn despawn(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<components::Bullet>>,
) {
    for (bullet, bullet_transform) in bullet_query.iter() {
        let limit = Vec2::splat(constants::MAP_SIZE as f32 / 2.);
        let bullet_pos_2d = bullet_transform.translation.xz();
        if bullet_pos_2d.clamp(-limit, limit) != bullet_pos_2d {
            commands.entity(bullet).despawn_recursive();
        }
    }
}
