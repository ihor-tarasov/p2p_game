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

        for mut transform in camera_query.iter_mut() {
            transform.look_at(pos, Vec3::Y);
        }
    }
}
