use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ModelAssets {
    #[asset(path = "tank_green.glb#Scene0")]
    pub tank_green: Handle<Scene>,
    #[asset(path = "tank_red.glb#Scene0")]
    pub tank_red: Handle<Scene>,
    #[asset(path = "bullet.glb#Scene0")]
    pub bullet: Handle<Scene>,
    #[asset(path = "arena.glb#Scene0")]
    pub arena: Handle<Scene>,
}
