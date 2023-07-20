use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ggrs::*;

use crate::*;

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
