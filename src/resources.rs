use bevy::prelude::*;

#[derive(Resource)]
pub struct LocalPlayerHandle(pub usize);

// Save players positions.
#[derive(Resource, Default)]
pub struct PlayersPositionsBackup(pub Vec<(usize, Vec2)>);
