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
    //images: Res<ImageAssets>,
    mut player_query: Query<(
        &Transform,
        &components::Player,
        &mut components::BulletReady,
        &components::MoveDir,
    )>,
    mut rip: ResMut<RollbackIdProvider>,
) {
    for (transform, player, mut bullet_ready, move_dir) in player_query.iter_mut() {
        let (input, _) = inputs[player.handle];
        if input::fire(input) && bullet_ready.0 {
            let player_pos = transform.translation.xy();
            let pos = player_pos + move_dir.0 * PLAYER_RADIUS + BULLET_RADIUS;
            commands.spawn((
                components::Bullet,
                rip.next(),
                *move_dir,
                SpriteBundle {
                    transform: Transform::from_translation(pos.extend(200.))
                        .with_rotation(Quat::from_rotation_arc_2d(Vec2::X, move_dir.0)),
                    //texture: images.bullet.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(0.3, 0.1)),
                        ..default()
                    },
                    ..default()
                },
            ));
            bullet_ready.0 = false;
        }
    }
}

pub fn move_bullet(
    mut query: Query<(&mut Transform, &components::MoveDir), With<components::Bullet>>,
) {
    for (mut transform, dir) in query.iter_mut() {
        let delta = (dir.0 * 0.35).extend(0.);
        transform.translation += delta;
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
                player_transform.translation.xy(),
                bullet_transform.translation.xy(),
            );
            if distance < PLAYER_RADIUS + BULLET_RADIUS {
                commands.entity(player).despawn_recursive();
            }
        }
    }
}
