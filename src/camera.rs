use bevy::prelude::*;

use crate::*;

pub fn camera_follow(
    player_handle: Option<Res<resources::LocalPlayerHandle>>,
    player_query: Query<(&components::Player, &Transform)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<components::Player>)>,
) {
    let player_handle = match player_handle {
        Some(handle) => handle.0,
        None => return, // Session hasn't started yet
    };

    for (player, player_transform) in player_query.iter() {
        if player.handle != player_handle {
            continue;
        }

        let pos = player_transform.translation;
        let rotation = player_transform.rotation;

        for mut transform in camera_query.iter_mut() {
            let movement_factor = Vec3::ONE;
            let movement_direction = rotation * Vec3::Z;
            let movement_distance = movement_factor * -5.0;
            let translation_delta = movement_direction * movement_distance;

            transform.translation = Vec3::new(pos.x, 5.0, pos.z) + translation_delta;

            transform.look_at(Vec3::new(pos.x, 2.5, pos.z), Vec3::Y);
        }
    }
}
