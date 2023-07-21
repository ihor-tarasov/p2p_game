use bevy::{pbr::PointLightShadowMap, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_ggrs::*;
use p2p_game::*;

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    networking::configure(&mut app);

    app.add_state::<states::GameState>()
        .add_loading_state(
            LoadingState::new(states::GameState::AssetLoading)
                .continue_to_state(states::GameState::Matchmaking),
        )
        .add_collection_to_loading_state::<_, assets::ModelAssets>(states::GameState::AssetLoading)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // fill the entire browser window
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(resources::PlayersPositionsBackup::default())
        .insert_resource(PointLightShadowMap { size: 4096 })
        .add_systems(
            (setup, networking::start_matchbox_socket)
                .in_schedule(OnEnter(states::GameState::Matchmaking)),
        )
        .add_systems((
            networking::wait_for_players.run_if(in_state(states::GameState::Matchmaking)),
            player::spawn_players.in_schedule(OnEnter(states::GameState::InGame)),
            camera::camera_follow.run_if(in_state(states::GameState::InGame)),
        ))
        .add_systems(
            (
                player::save_positions,
                player::move_players.after(player::save_positions),
                player::players_collision.after(player::move_players),
                bullet::reload_bullet,
                bullet::fire_bullets
                    .after(player::players_collision)
                    .after(bullet::reload_bullet),
                bullet::move_bullet.after(bullet::fire_bullets),
                bullet::kill_players
                    .after(bullet::move_bullet)
                    .after(player::players_collision),
                bullet::despawn.after(bullet::kill_players),
            )
                .in_schedule(GGRSSchedule),
        )
        .run();
}

fn setup(mut commands: Commands, models: Res<assets::ModelAssets>) {
    commands.spawn(SceneBundle {
        scene: models.arena.clone(),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            range: 41.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
